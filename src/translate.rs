use crate::const_var::*;
use crate::instruction::RiscInst;
use crate::parser::parse_instruction;
use std::mem::size_of;

pub fn translate_block(risc_addr: u64, c_info: u64) {
    const max_count: usize = BLOCK_CACHE_SIZE;
    let mut block_cache: Vec<RiscInst> = Vec::new();
    block_cache.resize_with(max_count, Default::default);
    // todo: float block
    let _instructions_in_block = parse_block(risc_addr, &mut block_cache, max_count, c_info);
}

fn parse_block(risc_addr: u64, parse_buf: &mut Vec<RiscInst>, max_count: usize, _c_info: u64) {
    println!("{:#?}", size_of::<RiscInst>());
    // todo: why -2 ?
    let mut curr_addr = risc_addr;
    for parse_pos in 0..(max_count - 2) {
        parse_buf[parse_pos].addr = curr_addr;
        parse_instruction(&mut parse_buf[parse_pos]);
    }
}
