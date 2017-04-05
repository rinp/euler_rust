
use num::PrimInt;

pub fn exe(mut power: usize) -> usize {

    let mut base = 2;

    loop {
        if power % 2 == 0 {
            base *= base;
            power /= 2;
        } else {
            break;
        }
    }

    let max_digits = max_digits(base, power);
    //    println!("最大桁数は:{}", max_digits);
    let mut v: Vec<usize> = vec![0; max_digits];
    v[0] = 1;
    for i in 0..power {
        for l in (0..max_digits).rev() {
            // println!("{} * {}", v[l], base);
            let n = v[l] * base;
            if 1000 <= n {
                v[l + 3] += n / 1000;
                v[l] = n % 1000;
            } else {
                v[l] = n;
            }
        }
        // println!("ループ:{}", i);
        // v.iter().rev().clone().fold((), |_, s| print!(" {}", s));
        // println!("");
    }
    return div_arrange(v, 1).iter().sum();
}

fn div_arrange(mut v: Vec<usize>, power: usize) -> Vec<usize> {
    let digits: usize = 10.pow(power as u32);
    for i in 0..v.len() {
        let n = v[i];
        if digits <= n {
            v[i + power] += n / digits;
            v[i] = n % digits;
        } else {
            v[i] = n;
        }

    }
    // println!("整地");
    // v.iter().rev().clone().fold((), |_, s| print!(" {}", s));
    // println!("");
    v
}

fn max_digits(base: usize, power: usize) -> usize {
    1 + (power as f32 * (base as f32).log10()) as usize
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use answer::*;

    #[test]
    fn max_digits_2_2() {
        assert_eq!(super::max_digits(2, 2), 1);
    }
    #[test]
    fn max_digits_2_4() {
        assert_eq!(super::max_digits(2, 4), 2);
    }
    #[test]
    fn max_digits_2_10() {
        assert_eq!(super::max_digits(2, 10), 4);
    }
    #[test]
    fn max_digits_10_3() {
        assert_eq!(super::max_digits(10, 3), 4);
    }
    #[test]
    fn test_exe_1() {
        assert_eq!(super::exe(15), 26);
    }

    #[test]
    fn test_exe_20() {
        assert_eq!(super::exe(20), 31);
    }

    #[test]
    fn test_exe_1000() {
        assert_eq!(super::exe(1000), ANSWER_16);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| super::exe(1000));
    }

}