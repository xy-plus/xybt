/* RISC-V ELF Flags */
pub const EF_RISCV_RVC: u32 = 0x0001;
pub const EF_RISCV_RVE: u32 = 0x0008;
pub const EF_RISCV_TSO: u32 = 0x0010;
pub const EF_RISCV_FLOAT_ABI: u32 = 0x0006;
pub const EF_RISCV_FLOAT_ABI_SOFT: u32 = 0x0000;
pub const EF_RISCV_FLOAT_ABI_SINGLE: u32 = 0x0002;
pub const EF_RISCV_FLOAT_ABI_DOUBLE: u32 = 0x0004;
pub const EF_RISCV_FLOAT_ABI_QUAD: u32 = 0x0006;

pub const TRANSLATOR_BASE: u64 = 0x780000000000;
// TODO: use rlimit to get stack size?
pub const STACK_OFFSET: u64 = 0x7f000000;
pub const USER_STACK_SIZE: u64 = 8 * 1024 * 1024;
pub const GUARD_PAGE_SIZE: u64 = 4096;
pub const USER_STACK_START_ADDR: u64 =
    TRANSLATOR_BASE - STACK_OFFSET - USER_STACK_SIZE - GUARD_PAGE_SIZE;

pub const AUXC: usize = 16;
pub const GENERAL_REGS_NUM: usize = 34;
