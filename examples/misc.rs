extern crate targeting;

use targeting::{Triple, HOST};
use std::str::FromStr;

fn main() {
    println!("The host triple is {}.", HOST);

    let e = Triple::from_str("riscv32-unknown-unknown")
        .expect("expected to recognize the RISC-V target")
        .endianness()
        .expect("expected to know the endianness of RISC-V");
    println!("The endianness of RISC-V is {:?}.", e);
}
