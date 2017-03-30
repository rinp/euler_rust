use common::iter;
pub fn exe(max: u64) -> usize {
    let prime_max: u64 = max / 6;
    let primes: Vec<u64> = iter::prime_iter().take_while(|i| i < &prime_max).collect();

    let count: usize = primes.iter()
        .clone()
        .filter_map(|&a: &u64| {

            let count_1: usize = if a * a * a * a * a * a * a < max {
                1
            } else {
                0
            };

            let count_23: usize = primes.iter()
                .clone()
                .skip_while(|&b| b <= &a)
                .filter_map(|&b: &u64| {


                    let count_3: usize = primes.iter()
                        .clone()
                        .skip_while(|&c| c <= &b)
                        .take_while(|&c| a * b * c < max)
                    //    .inspect(|c| println!("{} {} {}", a, b, c))
                        .count();


                    let a3b = a * a * a * b < max;
                    let ab3 = a * b * b * b < max;

                    let count_2 = if a3b && ab3 { 2 } else { 1 };
                    let sum = count_3 + count_2;

                    if sum != 0 { Some(sum) } else { None }

                })
                .sum();

            let sum = count_1 + count_23;
            if sum != 0 { Some(sum) } else { None }

        })
        .sum();

    count

}

#[cfg(test)]
mod tests {
    use answer;
    use super::*;
    use test::Bencher;


    #[test]
    fn test_exe_100() {
        assert_eq!(super::exe(100), 10);
    }

    #[bench]
    fn bench_exe_100(b: &mut Bencher) {
        b.iter(|| super::exe(100));
    }

    #[test]
    fn test_exe_1000() {
        assert_eq!(super::exe(1_000), 180);
    }

    #[bench]
    fn bench_exe_1000(b: &mut Bencher) {
        b.iter(|| super::exe(1_000));
    }

    // #[test]
    // fn test_exe_1000000() {
    //     assert_eq!(super::exe(1_000_000), 224427);
    // }

    // #[bench]
    // fn bench_exe_1000000(b: &mut Bencher) {
    //     b.iter(|| super::exe(1_000_000));
    // }

    // #[test]
    // fn test_exe_1000000000000() {
    //     assert_eq!(super::exe(100_0000_000_000), 224427);
    // }

    // #[bench]
    // fn bench_exe_1000000000000(b: &mut Bencher) {
    //     b.iter(|| super::exe(100_0000_000_000));
    // }
}
