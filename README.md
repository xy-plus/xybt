# run RISCV program on x86

[日志](./log.md)

## 编译用户程序

- C 程序（暂时没使用）

```sh
riscv64-unknown-linux-gnu-gcc hello.c -o hello.out -static -march=rv64imafd -mabi=lp64d
```

- 汇编程序

```sh
make riscv_user
```

## 运行

- 使用 xybt

```sh
make run
```

- 使用 qemu

```sh
qemu-riscv64 hello.out
qemu-riscv64 -d in_asm,out_asm hello.out  # 带一些信息
```

## 内存布局

- xybt

```
# 上方表示高地址，下方表示低地址
TRANSLATOR_BASE
USER_STACK_END_ADDR = TRANSLATOR_BASE - STACK_OFFSET
USER_CAN_USE_STACK_START_ADDR = USER_STACK_START_ADDR + GUARD_PAGE_SIZE = TRANSLATOR_BASE - STACK_OFFSET - USER_STACK_SIZE
USER_STACK_START_ADDR = TRANSLATOR_BASE - STACK_OFFSET - USER_STACK_SIZE - GUARD_PAGE_SIZE
```

- ria-jit

```
# 上方表示高地址，下方表示低地址
TRANSLATOR_BASE
InstrMem = TRANSLATOR_BASE - STACK_OFFSET
stack top = TRANSLATOR_BASE - STACK_OFFSET - 4096
guard protect end = bottomOfStack = TRANSLATOR_BASE - STACK_OFFSET - stackSize - 4096
stackStart = bottomOfStack = guard protect start = TRANSLATOR_BASE - STACK_OFFSET - stackSize - guard - 4096
```

## 计划

- [x] 读取 elf
- [x] 映射 elf 到内存
- [x] 把翻译器放到指定高地址
- [x] 为用户程序分配 stack
- [x] 拷贝 auxv、envp、argv、argc 到用户程序的栈中
- [x] 设置 inst mem
- [x] 翻译
- [x] 执行
- [ ] 保存上下文
- [ ] 缓存块
- [ ] 寄存器映射
- [ ] 其它
