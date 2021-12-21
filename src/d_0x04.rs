pub mod d0x04 {

    use std::fmt::Display;
    use std::fs;
    use std::str::FromStr;

    #[derive(Clone, Copy)]
    pub struct Number {
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
            )?;
            Ok(())
        }
    }

    const TABLE_SIZE: usize = 5;

    #[derive(Clone)]
    pub struct Table {
        data: [[Number; TABLE_SIZE]; TABLE_SIZE],
        score: i32,
    }

    impl Display for Table {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            for (_x, row) in self.data.iter().enumerate() {
                for (_y, col) in row.iter().enumerate() {
                    write!(f, "{} ", col)?;
                }
                writeln!(f)?;
            }

            writeln!(f, "Score: {}\n", self.score)?;
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

    enum Where {
        Column,
        Row,
    }

    struct Win {
        position: Where,
        index: usize,
    }

    impl Table {
        fn has_won(&self) -> Option<Win> {
            // Check rows
            for (x, row) in self.data.iter().enumerate() {
                if row.iter().filter(|n| n.selected).count() == TABLE_SIZE {
                    return Some(Win {
                        position: Where::Row,
                        index: x,
                    });
                }
            }

            // `i` is the column index
            for i in 0..TABLE_SIZE {
                let mut lost = false;
                for (_x, row) in self.data.iter().enumerate() {
                    if !row[i].selected {
                        lost = true;
                    }
                }

                if !lost {
                    return Some(Win {
                        position: Where::Column,
                        index: i,
                    });
                }
            }

            None
        }

        fn compute_score(&self) -> i32 {
            let mut score = 0;
            for (_x, row) in self.data.iter().enumerate() {
                for (_y, col) in row.iter().enumerate() {
                    if !col.selected {
                        score += col.value;
                    }
                }
            }
            score
        }

        fn put_number(&mut self, n: i32) {
            if self.score != 0 {
                return;
            }
            self.data.iter_mut().for_each(|row| {
                row.iter_mut()
                    .filter(|v| v.value == n)
                    .for_each(|v| v.selected = true);
            });

            match self.has_won() {
                Some(_win) => {
                    self.score = self.compute_score();
                }
                None => {}
            }
        }
    }

    pub fn parse_input(file: &str) -> (Vec<Table>, Vec<i32>) {
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

        (t, nums)
    }

    pub fn part1(input: &(Vec<Table>, Vec<i32>)) -> i32 {
        let (tables_immut, nums) = input;
        let mut tables = tables_immut.clone();
        let mut score = 0;
        nums.iter().for_each(|n| {
            if score != 0 {
                return;
            }
            tables.iter_mut().for_each(|t| {
                t.put_number(*n);
                if t.score != 0 {
                    score = t.score * n;
                    return;
                }
            });
        });

        score
    }

    pub fn part2(input: &(Vec<Table>, Vec<i32>)) -> i32 {
        let (tables_immut, nums) = input;
        let mut tables = tables_immut.clone();

        let mut last: Table = Table {
            data: [[Number {
                value: 0,
                selected: false,
            }; TABLE_SIZE]; TABLE_SIZE],
            score: 0,
        };

        let mut score = 0;
        let mut last_n = 0;

        nums.iter().for_each(|n| {
            if score != 0 {
                return;
            }

            tables.iter_mut().for_each(|table| {
                let prev_score = table.score;
                table.put_number(*n);
                if (table.score != prev_score) && (table.score != 0) {
                    last = table.clone();
                    last_n = *n;
                }
            });

            if tables.iter().filter(|t| t.score == 0).count() == 0 {
                score = last.compute_score() * last_n;
            }
        });

        score
    }
}
