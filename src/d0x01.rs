pub fn d0x01_p1(input: &Vec<i32>) -> i32 {
    let mut prev = -1;
    let mut count = 0;

    for e in input {
        if prev != -1 && *e > prev {
            count += 1;
        }
        prev = *e;
    }

    count
}

pub fn d0x01_p2(input: &Vec<i32>) -> i32 {
    let mut i = 0;
    let mut count = 0;

    loop {
        if input.len() - 3 <= i {
            break count;
        }

        let s1 = input[i] + input[i + 1] + input[i + 2];
        let s2 = input[i + 1] + input[i + 1 + 1] + input[i + 2 + 1];

        if s2 > s1 {
            count += 1;
        }

        i += 1;
    }
}
