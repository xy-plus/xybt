use crate::const_var::*;
use crate::instruction::{OpType, RiscInst, MNEM};
use crate::parser::parse_instruction;
use std::mem::size_of;

fn translate_risc_instr(inst: &RiscInst, c_info: u64) {
    match &inst.mnem {
        // MNEM::ADDI => {}
        mismatch_optype => {
            panic!("mismatch optype {:?}", mismatch_optype);
        }
    }
}

fn translate_block_instructions(
    block_cache: &Vec<RiscInst>,
    instructions_in_block: u64,
    c_info: u64,
) {
    // init_block(c_info->r_info);  // todo: use it
    // todo: opt inst
    for i in 0..instructions_in_block {
        translate_risc_instr(&block_cache[i as usize], c_info);
    }
}
pub fn translate_block(risc_addr: u64, c_info: u64) {
    const max_count: usize = BLOCK_CACHE_SIZE;
    let mut block_cache: Vec<RiscInst> = Vec::new();
    block_cache.resize_with(max_count, Default::default);
    // todo: float block
    let instructions_in_block = parse_block(risc_addr, &mut block_cache, max_count, c_info);
    let block = translate_block_instructions(&block_cache, instructions_in_block, c_info);
}

fn parse_block(
    risc_addr: u64,
    parse_buf: &mut Vec<RiscInst>,
    max_count: usize,
    _c_info: u64,
) -> u64 {
    println!("{:#?}", size_of::<RiscInst>());
    // todo: why -2 ?
    let mut curr_addr = risc_addr;
    let mut instructions_in_block = 0;

    for parse_pos in 0..(max_count - 2) {
        parse_buf[parse_pos].addr = curr_addr;
        parse_instruction(&mut parse_buf[parse_pos]);
        match &parse_buf[parse_pos].optype {
            OpType::IMMEDIATE => {
                curr_addr += 4;
                instructions_in_block += 1;
            }
            OpType::SYSTEM => match &parse_buf[parse_pos].mnem {
                MNEM::ECALL => {
                    instructions_in_block += 1;
                    return instructions_in_block;
                }
                mismatch_optype => {
                    panic!("mismatch optype {:?}", mismatch_optype);
                }
            },
            mismatch_optype => {
                panic!("mismatch optype {:?}", mismatch_optype);
            }
        }
    }
    return instructions_in_block;
}
