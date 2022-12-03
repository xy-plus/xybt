# qemu tcg 源码分析

## tcg 调用

```c
TranslationBlock *tb_gen_code(CPUState *cpu,
                              target_ulong pc, target_ulong cs_base,
                              uint32_t flags, int cflags)
{
    tb = tb_alloc(pc);
...
    /* generate IR code */
    gen_intermediate_code(cpu, tb, max_insns);
...
    /* generate machine code */
    gen_code_size = tcg_gen_code(tcg_ctx, tb);
...
}
```

## translator_loop

虽然 translator_loop 对于任何架构来说都是一样的，但它依赖于目标特定的转换器操作符。

```c
void translator_loop(const TranslatorOps *ops, DisasContextBase *db,
                     CPUState *cpu, TranslationBlock *tb, int max_insns)
{
    ops->init_disas_context(db, cpu);
...
    gen_tb_start(db->tb);
    ops->tb_start(db, cpu);

    while (true) {
        ops->translate_insn(db, cpu);
    }
...
    ops->tb_stop(db, cpu);
    gen_tb_end(db->tb, db->num_insns - bp_insn);
}
```

每个 TB 都有一个开头 (tb_start) 和一个结尾 (tb_end)。通常尾声是块链接的占位符，这是 TCG 的优化特性，可以在执行后连续调用 TB，而无需返回 QEMU 代码并寻找下一个 TB 执行。

## 示例 PowerPC 基本块转换

下面的 PowerPC 代码显示了 3 种类型的操作：

- 简单指令：算术、立即操作数 (lis, ori, xor)
- 内存写入 (stw)
- 系统寄存器写入 (mtmsr)

```c
0xfff00100:  lis    r1,1

0xfff00104:  ori    r1,r1,0x409c

0xfff00108:  xor    r0,r0,r0

0xfff0010c:  stw    r0,4(r1)

0xfff00110:  mtmsr  r0
```

以下代码给出了 TCG IR 等效项：

```c
0xfff00100:  movi_i32    r1,$0x10000

0xfff00104:  movi_i32    tmp0,$0x409c
             or_i32      r1,r1,tmp0

0xfff00108:  movi_i32    r0,$0x0

0xfff0010c:  movi_i32    tmp1,$0x4
             add_i32     tmp0,r1,tmp1
             qemu_st_i32 r0,tmp0,beul,3

0xfff00110:  movi_i32    nip,$0xfff00114
             mov_i32     tmp0,r0
             call        store_msr,$0,tmp0
             movi_i32    nip,$0xfff00114
             exit_tb     $0x0
             set_label   $L0
             exit_tb     $0x7f5a0caf8043
```

我们注意到内存写入操作以及 MSR 访问被转换为奇怪的 IR 操作码：

- qemu_st_i32
- call store_msr

还要注意操作 exit_tb 码的操作数。第一个是 $0x0 并且可能最终被修改为下一个要执行的 TB 的绝对主机地址。

最后一个也是主机地址($0x7f5a0caf8043)，代码由物理 CPU 直接执行。在这种情况下，它会返回到 QEMU 转换器。
