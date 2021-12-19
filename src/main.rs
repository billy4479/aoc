pub mod common;

pub mod d_0x01;
pub mod d_0x02;
pub mod d_0x03;

use d_0x01::d0x01;
use d_0x02::d0x02;
use d_0x03::d0x03;

macro_rules! solve_day {
    ($m:tt) => {
        let mod_name = stringify!($m);
        let input = $m::parse_input(format!("input/{}.txt", mod_name).as_str());
        println!("{}:", mod_name);
        let r1 = $m::part1(&input);
        println!("\t{}", r1);
        let r2 = $m::part2(&input);
        println!("\t{}", r2);
    };
}

fn main() {
    println!("Advent of Code!");

    solve_day!(d0x01);
    solve_day!(d0x02);
    solve_day!(d0x03);
}
