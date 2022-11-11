# run RISCV program on x86

[日志](./log.md)

## 计划

- [x] 读取 elf
- [x] 映射 elf 到内存
- [x] 把翻译器放到指定高地址
- [x] 为用户程序分配 stack
- [x] 拷贝 auxv、envp、argv、argc 到用户程序的栈中
- [ ] 初始化用户寄存器
- [ ] 翻译
- [ ] 执行
- [ ] 其它
