pub mod d0x02 {

    use crate::common;

    pub fn parse_input(file: &str) -> Vec<String> {
        common::parse_input_file_as_vec(file)
    }

    pub fn part1(input: &Vec<String>) -> i32 {
        let mut depth = 0;
        let mut hp = 0;

        for e in input {
            let mut iter = e.split_whitespace();
            let cmd = iter.next().unwrap();
            let n = iter.next().unwrap().parse::<i32>().unwrap();

            match cmd {
                "forward" => {
                    hp += n;
                }
                "down" => {
                    depth += n;
                }
                "up" => {
                    depth -= n;
                }
                _ => {}
            };
        }

        depth * hp
    }

    pub fn part2(input: &Vec<String>) -> i32 {
        let mut depth = 0;
        let mut hp = 0;
        let mut aim = 0;

        for e in input {
            let mut iter = e.split_whitespace();
            let cmd = iter.next().unwrap();
            let n = iter.next().unwrap().parse::<i32>().unwrap();

            match cmd {
                "forward" => {
                    hp += n;
                    depth += n * aim;
                }
                "down" => {
                    aim += n;
                }
                "up" => {
                    aim -= n;
                }
                _ => {}
            };
        }

        depth * hp
    }
}
