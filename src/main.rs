pub mod const_var;
mod read_elf;
mod utils;
use read_elf::read_elf;
use std::env;

fn main() {
    let mapped_elf = read_elf(&env::args().nth(1).expect("no input file"));
    println!("{:#x?}", mapped_elf);
}
