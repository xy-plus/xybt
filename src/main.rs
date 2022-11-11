pub mod const_var;
mod memory;
mod read_elf;
mod registers;
mod utils;
use read_elf::read_elf;
pub use read_elf::MappedElf;
use registers::{REG, REGISTERS};
use std::env;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    assert!(args.len() > 1, "no input file");
    let mapped_elf = read_elf(&args[1]);
    // println!("{:#x?}", mapped_elf);
    args.remove(0);
    let env_vars: Vec<String> = env::vars().map(|(k, v)| [k, v].join("=")).collect();
    let user_program_stack_addr = memory::create_user_stack(mapped_elf, &args, &env_vars);
    // println!("{:#x?}", user_program_stack_addr);
    let mut registers = REGISTERS::new();
    registers.set_reg(REG::sp, user_program_stack_addr);
    println!("{:#x?}", registers);
}
