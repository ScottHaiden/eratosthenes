use std::io::{BufWriter, Write};

fn to_ind(i: usize) -> usize { return (i - 3) / 2; }

fn main() {
    let mut out = BufWriter::new(std::io::stdout());

    let len = to_ind(u32::MAX as usize) + 1;
    let mut nums = vec![true; len];

    for i in (3..u32::MAX as usize).step_by(2) {
        if !nums[to_ind(i)] { continue; }

        let bytes = (i as u32).to_be_bytes();
        out.write_all(&bytes).expect("write failed");

        for j in ((i * 3)..(u32::MAX as usize)).step_by(i * 2) {
            nums[to_ind(j)] = false;
        }
    }
}
