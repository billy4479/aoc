pub mod common;

pub mod d0x01;
pub mod d0x02;

use d0x01::*;
use d0x02::*;

fn main() {
    println!("Advent of Code!");
    {
        let input = common::read_input("input/0x01.txt");
        let r1 = d0x01_p1(&input);
        println!("d0x01_p1: {}", r1);
        let r2 = d0x01_p2(&input);
        println!("d0x01_p2: {}", r2);
    }
    {
        let input = common::read_input("input/0x02.txt");
        let r1 = d0x02_p1(&input);
        println!("d0x02_p1: {}", r1);
        let r2 = d0x02_p2(&input);
        println!("d0x02_p2: {}", r2);
    }
}
