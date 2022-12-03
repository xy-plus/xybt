# qemu 源码分析

## 流程

通过 qemu_init_vcpu() 启动模拟器。

假设我们使用单线程 TCG 运行 QEMU，并且没有硬件辅助虚拟化后端，我们最终将在专用线程中运行我们的虚拟 CPU 。

```c
static void qemu_tcg_init_vcpu(CPUState *cpu)
{
...
    qemu_thread_create(cpu->thread, thread_name,
                       qemu_tcg_rr_cpu_thread_fn,
                       cpu, QEMU_THREAD_JOINABLE);
...
}

static void *qemu_tcg_rr_cpu_thread_fn(void *arg)
{
...
    while (1) {
        while (cpu && !cpu->queued_work_first && !cpu->exit_request) {

            qemu_clock_enable(QEMU_CLOCK_VIRTUAL, ...);

            if (cpu_can_run(cpu)) {
                r = tcg_cpu_exec(cpu);

                if (r == EXCP_DEBUG) {
                    cpu_handle_guest_debug(cpu);
                    break;
                }
            }
            cpu = CPU_NEXT(cpu);
        }
    }
}
```

如果 vCPU 处于可运行状态，则我们通过 TCG 执行指令。

## TCG

tcg_cpu_exec 是循环的主体。

```c
int cpu_exec(CPUState *cpu)
{
    cc->cpu_exec_enter(cpu);

    /* prepare setjmp context for exception handling */
    sigsetjmp(cpu->jmp_env, 0);

    /* if an exception is pending, we execute it here */
    while (!cpu_handle_exception(cpu, &ret)) {
        while (!cpu_handle_interrupt(cpu, &last_tb)) {
            tb = tb_find(cpu, last_tb, tb_exit, cflags);
            cpu_loop_exec_tb(cpu, tb, &last_tb, &tb_exit);
        }
    }

    cc->cpu_exec_exit(cpu);
}
```

QEMU 利用 setjmp/longjmp 来实现异常处理。这允许在触发事件（例如 CPU 中断或异常）时退出深度和复杂的 TCG 转换函数。退出 CPU 执行循环的相应函数是 cpu_loop_exit_xxx：

```c
void cpu_loop_exit(CPUState *cpu)
{
    /* Undo the setting in cpu_tb_exec.  */
    cpu->can_do_io = 1;
    siglongjmp(cpu->jmp_env, 1);
}
```

vCPU 线程代码执行回到它调用的点 sigsetjmp。然后 QEMU 尝试尽快处理该事件。但如果没有待处理的，它会执行所谓的翻译块（TB）。

## 翻译块

TCG 引擎是一个 JIT 编译器，这意味着它将目标架构指令集动态转换为主机架构指令集。翻译分两步完成：

从目标 ISA 到中间表示 (IR)
从 IR 到主机 ISA

QEMU 首先尝试查找现有的 TB，使用 tb_find. 如果当前位置不存在，则生成一个新位置 tb_gen_code：

```c
static inline TranslationBlock *tb_find(CPUState *cpu,
                                        TranslationBlock *last_tb,
                                        int tb_exit, uint32_t cf_mask)
{
...
    tb = tb_lookup__cpu_state(cpu, &pc, &cs_base, &flags, cf_mask);
    if (tb == NULL) {
        tb = tb_gen_code(cpu, pc, cs_base, flags, cf_mask);
...
}
```

当一个 TB 可用时，cpu_loop_exec_tb 会在短调用中运行 cpu_tb_exec ，然后调用 tcg_qemu_tb_exec.。此时目标代码已经被翻译成主机代码，QEMU 可以直接在主机 CPU 上运行它。如果我们看一下最后一个函数的定义：

```c
#define tcg_qemu_tb_exec(env, tb_ptr) \
    ((uintptr_t (*)(void *, void *))tcg_ctx->code_gen_prologue)(env, tb_ptr)
```

接收生成的操作码的翻译缓冲区被转换为函数指针并使用参数调用。

## 返回事件处理

当引发硬件中断 (IRQ) 或异常时，QEMU 会帮助 vCPU 将执行重定向到适当的处理程序。这些机制和架构强相关，所以很难翻译。好像用了 C 编写的微型包装器来处理，暂时没读懂。
