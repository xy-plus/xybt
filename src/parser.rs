use crate::instruction::RiscInst;

pub fn parse_instruction(parse_buf: &mut RiscInst) {
    parse_buf.optype = 6;
    println!("{:#x?}", parse_buf);
}
