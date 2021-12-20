pub mod d0x04 {

    use std::fmt::Display;
    use std::fs;
    use std::str::FromStr;

    #[derive(Clone, Copy)]
    struct Number {
        value: i32,
        selected: bool,
    }

    impl Display for Number {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "{}{:>2}",
                if self.selected { "!" } else { " " },
                self.value
            );
            Ok(())
        }
    }

    const TABLE_SIZE: usize = 5;

    struct Table {
        data: [[Number; TABLE_SIZE]; TABLE_SIZE],
        score: i32,
    }

    impl Display for Table {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            for (_x, row) in self.data.iter().enumerate() {
                for (_y, col) in row.iter().enumerate() {
                    write!(f, "{}", col);
                }
                writeln!(f);
            }
            Ok(())
        }
    }

    impl FromStr for Table {
        type Err = std::string::ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut t = Table {
                data: [[Number {
                    value: 0,
                    selected: false,
                }; 5]; 5],
                score: 0,
            };
            let table_nums = s
                .split(&[' ', '\n'][..])
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut i = 0;

            for (_x, row) in t.data.iter_mut().enumerate() {
                for (_y, col) in row.iter_mut().enumerate() {
                    col.value = table_nums[i];
                    i += 1;
                }
            }

            Ok(t)
        }
    }

    pub struct InputData(Vec<Table>, Vec<i32>);

    pub fn parse_input(file: &str) -> InputData {
        let s = fs::read_to_string(file).unwrap();
        let mut iter = s.split("\n\n");
        let nums = iter
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i32>>();

        let t = iter
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<Table>>();

        InputData { 0: t, 1: nums }
    }

    pub fn part1(input: &InputData) -> i32 {
        0
    }

    pub fn part2(input: &InputData) -> i32 {
        0
    }
}
