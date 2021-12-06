pub mod d0x02 {

    pub fn d0x02_p1(input: Vec<String>) -> i32 {
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

    pub fn d0x02_p2(input: Vec<String>) -> i32 {
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

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::common::*;

        #[test]
        fn d0x02_p1t() {
            let input = read_input("input/0x02.txt");
            let result = d0x02_p1(input);
            println!("{}", result);
        }

        #[test]
        fn d0x02_p2t() {
            let input = read_input("input/0x02.txt");
            let result = d0x02_p2(input);
            println!("{}", result);
        }
    }
}
