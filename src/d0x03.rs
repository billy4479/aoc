pub fn d0x03_p1(input: &Vec<String>) -> u32 {
    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;
    let mut c0: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for e in input.iter() {
        let n = u16::from_str_radix(e.as_str(), 2).unwrap();

        for count in c0.iter_mut().enumerate() {
            let (i, c): (usize, &mut u32) = count;

            if (n & (1 << i)) > 0 {
                *c += 1;
            }
        }
    }

    for count in c0.iter().enumerate() {
        let (i, c): (usize, &u32) = count;

        if *c > (input.len() / 2) as u32 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    gamma as u32 * epsilon as u32
}

pub fn d0x03_p2(input: &Vec<String>) -> u32 {
    let v = input
        .iter()
        .map(|raw| u16::from_str_radix(raw.as_str(), 2).unwrap())
        .collect::<Vec<u16>>();

    d0x03_p2_u1(&v, 0) * d0x03_p2_u1(&v, 1)
}

fn d0x03_p2_u1(input: &Vec<u16>, criteria: u16) -> u32 {
    let mut position = 0;
    let mut v = input.clone();

    while v.len() > 1 {
        let mut count1 = 0;
        for elem in &v {
            count1 += d0x03_p2_u2(elem, position) as usize;
        }
        let count0 = v.len() - count1;

        /*
         * CO2 = 0
         * O2 = 1
         *
         * O2 -> more common
         * CO2 -> less common
         */

        let n: u16 = if count1 >= count0 {
            // 1 is more common or equal
            criteria
        } else {
            // 0 is more common
            criteria ^ 0b01 // invert the last bit
        };

        v = v
            .into_iter()
            .filter(|val| d0x03_p2_u2(val, position) == n)
            .collect::<Vec<u16>>();
        position += 1;
    }

    v[0] as u32
}

fn d0x03_p2_u2(elem: &u16, position:u16)-> u16{
    
        (elem & 
            // Get the mask for bit at `position` from left
            (1 << (11- position))
        ) 
        // Shift back the selected bit to be the first one of the number
        >> (11 - position)
    
}
