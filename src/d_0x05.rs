pub mod d0x05 {

    use std::cmp;
    use std::fmt::Display;
    use std::fs;
    use std::str::FromStr;

    pub struct Line {
        x1: i32,
        x2: i32,
        y1: i32,
        y2: i32,
    }

    impl FromStr for Line {
        type Err = std::string::ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (p1, p2) = s.split_once(" -> ").unwrap();

            let (x1, y1) = p1.split_once(",").unwrap();
            let (x2, y2) = p2.split_once(",").unwrap();

            Ok(Line {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
            })
        }
    }

    impl Display for Line {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "[{:>3} : {:>3}] -> [{:>3} : {:>3}]",
                self.x1, self.y1, self.x2, self.y2
            )?;

            Ok(())
        }
    }

    pub fn parse_input(file: &str) -> Vec<Line> {
        fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| Line::from_str(l).unwrap())
            .collect()
    }

    fn find_max(input: &Vec<Line>) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;

        input.iter().for_each(|l| {
            max_x = cmp::max(max_x, l.x1);
            max_x = cmp::max(max_x, l.x2);
            max_y = cmp::max(max_y, l.y1);
            max_y = cmp::max(max_y, l.y2);
        });

        (max_x as usize, max_y as usize)
    }

    fn print_grid(grid: &Vec<Vec<i32>>) {
        let mut x = 0;
        grid.iter().for_each(|row| {
            print!("{:<3}| ", x);
            row.iter().for_each(|n| {
                if *n != 0 {
                    print!("{}", n)
                } else {
                    print!(" ")
                }
            });
            println!();
            x += 1;
        })
    }

    pub fn part1(input: &Vec<Line>) -> i32 {
        let (max_x, max_y) = find_max(input);
        let mut grid = vec![vec![0; max_x + 1]; max_y + 1];

        input
            .iter()
            .filter(|l| l.x1 == l.x2 || l.y1 == l.y2)
            .for_each(|l| {
                if l.y1 == l.y2 {
                    let (start, end) = if l.x1 <= l.x2 {
                        (l.x1, l.x2)
                    } else {
                        (l.x2, l.x1)
                    };

                    for x in start..=end {
                        grid[l.y1 as usize][x as usize] += 1;
                    }
                } else {
                    let (start, end) = if l.y1 <= l.y2 {
                        (l.y1, l.y2)
                    } else {
                        (l.y2, l.y1)
                    };

                    for y in start..=end {
                        grid[y as usize][l.x1 as usize] += 1;
                    }
                }
            });

        // print_grid(&grid);

        let mut counter = 0;
        grid.iter()
            .for_each(|row| counter += row.iter().filter(|v| **v >= 2).count());

        counter as i32
    }

    pub fn part2(_input: &Vec<Line>) -> i32 {
        0
    }
}
