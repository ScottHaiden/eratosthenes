use std::io::{BufWriter, Stdout, Write};

fn to_ind(i: usize) -> usize { return (i - 3) / 2; }

fn print_prime(out: &mut BufWriter<Stdout>, prime: u32) {
    out.write_all(&prime.to_be_bytes()).expect("write failed");
}

fn mark_composites(nums: &mut Vec<bool>, prime: usize) {
    const MAX: usize = u32::MAX as usize;

    let square: usize = prime * prime;
    let double: usize = prime + prime;

    for i in (square..MAX).step_by(double) {
        nums[to_ind(i)] = true;
    }
}

fn main() {
    let mut out = BufWriter::new(std::io::stdout());

    let len = to_ind(u32::MAX as usize) + 1;
    let mut nums = vec![false; len];

    const LOW: usize = 3usize;
    const HIGH: usize = u32::MAX as usize;

    for i in (LOW..HIGH).step_by(2) {
        if nums[to_ind(i)] { continue; }
        print_prime(&mut out, i as u32);
        mark_composites(&mut nums, i);
    }
}
