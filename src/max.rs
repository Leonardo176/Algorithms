use std::mem::swap;

pub fn max_2(vec: &Vec<u32>) -> (u32, u32) {
    let mut counter = 1;
    let (mut max_1, mut max_2) = (vec[0], vec[1]);

    if max_1 < max_2 {
        swap(&mut max_1, &mut max_2);
    }

    for &n in vec.iter().skip(2) {
        let mut inc = 1;

        if n > max_2 {
            if n > max_1 {
                max_2 = max_1;
                max_1 = n;
            } else {
                max_2 = n;
            }
            inc += 1;
        }

        counter += inc;
    }

    println!("Num confronti: {counter}");

    (max_1, max_2)
}

pub fn max_2_opt(vec: &Vec<u32>) -> (u32, u32) {
    let mut counter = 1;
    let (mut max_1, mut max_2) = (vec[0], vec[1]);

    if max_1 < max_2 {
        swap(&mut max_1, &mut max_2);
    }

    for &n in vec.iter().skip(2) {
        let mut inc = 2;
        if n > max_1 {
            max_2 = max_1;
            max_1 = n;
            inc = 1;
        } else if n > max_2 {
            max_2 = n;
        }

        counter += inc;
    }

    println!("Num confronti: {counter}");

    (max_1, max_2)
}
