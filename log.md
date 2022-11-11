# 工作日志

## 2022-11-10

### 拷贝 auxv、envp、argv、argc 到用户栈中

注意栈指针需要 16 位对齐。

## 2022-11-09

### 把 translator 放入指定地址

尝试通过 rcore tutorial 那样加 linker.ld 的方案，不知为何会导致 libc 编译错误，修改无果。

尝试直接通过编译参数把程序放入高地址，然后找到的命令无法使用，且 rust 和 c 的编译存在一些 gap 。

后来找到了 c 里设置 text 段的地址的方案（因 gcc 版本更替导致命令改过名，我使用 `-Ttext 0x780000000000` ，网上的都是 `-Ttext-segmengt 0x780000000000`），但是无法将地址设置得太大。

最终解决方案：

```
[target.x86_64-unknown-linux-gnu]
rustflags = [
    "-Clink-args=-pie -fPIE -Wl,-Ttext-segment=0x780000000000",
]
```

通过 `readelf a.out -S` 可以看到程序被放到了正确的地址。

## 2022-11-08

### 把 elf 映射到内存中

通过 mmap 读出 elf 的中 load 类型的 segment ，这部分是需要加载到内存中的。记录最高地址和最低地址，把这中间的部分通过 mmap 分配给翻译器，然后把 segment 拷贝到相应的地址。

## 2022-11-07

### 初步读取 elf

学习了如何读 elf header、program header、section header，以及部分变量的作用。

后续好像会用到 phdr 的起始地址，是用 elf 的开始地址加上 elf_hdr.e_phoff 算出来的。其中 elf 的开始地址是用第一个 segment 的 program_hdr.p_vaddr - program_hdr.p_offset 算出来的。（理论上每个一 segment 算出来的这个值应该一样，但是实际上缺不一样，也许是我理解的有问题，望指正）第一个 senment 的地址是最低的。
