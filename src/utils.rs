pub fn align_up(v: u64, a: u64) -> u64 {
    ((v) + ((a) - 1)) & !((a) - 1)
}

pub fn align_down(v: u64, a: u64) -> u64 {
    (v) & !((a) - 1)
}
