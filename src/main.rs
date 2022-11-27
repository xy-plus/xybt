pub mod const_var;
mod memory;
mod read_elf;
mod registers;
mod utils;
use const_var::*;
use read_elf::read_elf;
pub use read_elf::MappedElf;
pub use registers::{init_map_context, RiscReg, REGISTERS};
use std::env;
mod translate;
use translate::translate_block;
pub mod instruction;
pub mod parser;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    assert!(args.len() > 1, "no input file");
    let mapped_elf = read_elf(&args[1]);
    // println!("{:#x?}", mapped_elf);
    // todo: setupBrk(result.dataEnd);
    args.remove(0);
    let env_vars: Vec<String> = env::vars().map(|(k, v)| [k, v].join("=")).collect();
    let user_program_stack_addr = memory::create_user_stack(&mapped_elf, &args, &env_vars);
    // println!("{:#x?}", user_program_stack_addr);
    let mut registers = REGISTERS::new();
    registers.set_reg(RiscReg::sp, user_program_stack_addr);
    println!("{:#x?}", registers);
    // todo: init_hash_table();
    // todo: init_return_stack();
    let inst_mem_addr = memory::setup_inst_mem();
    println!("{:#x?}", inst_mem_addr);
    // todo: context_info *c_info = init_map_context(result.floatBinary);

    let next_pc = mapped_elf.entry;
    registers.set_reg(RiscReg::pc, next_pc);
    let r_info = init_map_context(registers, inst_mem_addr);
    let _last_hint = TRANSLATOR_BASE - STACK_OFFSET - USER_STACK_SIZE;
    let mut exit = false;
    loop {
        if exit {
            break;
        }
        let _cache_loc = translate_block(next_pc, &r_info);
        // set_cache_entry(next_pc, cache_loc);
        exit = true;
    }
}
