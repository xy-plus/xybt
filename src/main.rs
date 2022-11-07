pub mod const_var;
mod read_elf;
use read_elf::read_elf;
use std::env;

fn main() {
    read_elf(env::args().nth(1).expect("no input file"));
}
