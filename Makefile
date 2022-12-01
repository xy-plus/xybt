.PHONY: clean

riscv_user: hello_asm/hello_riscv.S
	riscv64-unknown-linux-gnu-as hello_asm/hello_riscv.S -march=rv64imafd -mabi=lp64d -o hello_asm/hello_riscv.o
	riscv64-unknown-linux-gnu-ld -static hello_asm/hello_riscv.o -o hello.out

hello_x86: hello_asm/hello_x86.asm
	nasm -felf64 hello_asm/hello_x86.asm && ld hello_asm/hello_x86.o -o hello_asm/hello_x86.out && ./hello_asm/hello_x86.out

clean:
	cargo clean
	rm -f ./*.out
	rm -f hello_asm/*.o
	rm -f hello_asm/*.out

run: riscv_user
	cargo run hello.out
