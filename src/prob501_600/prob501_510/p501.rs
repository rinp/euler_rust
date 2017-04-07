// use std::collections::HashMap;
use common::iter;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use tempfile::NamedTempFileOptions;
use std::io::Write;
use std::io::BufReader;
use tempfile::NamedTempFile;
use std::io::BufRead;

fn prim_vec(max: &u64) -> NamedTempFile {
    let mut named_temp_file = NamedTempFileOptions::new()
        .prefix("prime_")
        .create_in("D:\\")
        .unwrap();

    let prime_max: u64 = max / 6;
    for i in iter::prime_iter().take_while(|i| i < &prime_max) {
        writeln!(&mut named_temp_file, "{}", i);

    }

    named_temp_file
}

pub fn exe(max: u64) -> u64 {

    let tmp_file = prim_vec(&max);
    //;
    let primes = BufReader::new(tmp_file.reopen().unwrap())
        .lines()
        .map(|l| &l.unwrap().parse::<u64>().unwrap());

    println!("素数生成完了");


    let count_1: u64 = (&primes.cloned()).take_while(|&a| a * a * a * a * a * a * a < max)
//        .inspect(|x| println!("{}", x))
        .count() as u64;


    let count_2: u64 = (&primes.cloned())
        .map(|a: u64| {
            println!("2素数:{}", a);

            (&primes.cloned())
                .skip_while(|&b| b <= a)
                .take_while(|&b| a * a * a * b < max)
                .count() as u64 +
            (&primes.cloned())
                .skip_while(|&b| b <= a)
                .take_while(|&b| a * b * b * b < max)
                .count() as u64
        })
        .sum::<u64>();


    let count_3: u64 = (&primes.cloned())
        .map(|a: u64| {
            println!("3素数:{}", a);

            primes.cloned()
                .skip_while(|&b| b <= a)
                .map(|b: u64| {
                    primes.cloned()
                        .skip_while(|&c| c <= b)
                        .take_while(|&c| a * b * c < max)
                 //       .inspect(|c| println!("{} {} {}", a, b, c))
                        .count() as u64
                })
                .take_while(|&sum| 0 < sum)
                .sum::<u64>()
        })
        .take_while(|&sum| 0 < sum)
        .sum();

    count_1 + count_2 + count_3

}

#[cfg(test)]
mod tests {
    // use answer;
    use test::Bencher;

    #[test]
    fn exe_100_test() {
        assert_eq!(super::exe(100), 10);
    }


    #[bench]
    fn exe_100_bench(b: &mut Bencher) {
        b.iter(|| super::exe(100));
    }

    #[test]
    fn exe_1_000_test() {
        assert_eq!(super::exe(1_000), 180);
    }

    #[bench]
    fn exe_1_000_bench(b: &mut Bencher) {
        b.iter(|| super::exe(1_000));
    }
    // test prob501_600::prob501_510::p501::tests::exe_100_bench       ... bench:       1,280 ns/iter (+/- 57)
    // test prob501_600::prob501_510::p501::tests::exe_1_000_bench     ... bench:      19,947 ns/iter (+/- 1,365)

    #[bench]
    fn bench_exe_10_000(b: &mut Bencher) {
        b.iter(|| super::exe(10_000));
    }

    #[test]
    fn test_exe_1_000_000() {
        assert_eq!(super::exe(1_000_000), 224427);
    }

    #[bench]
    fn exe_1_000_000_bench(b: &mut Bencher) {
        b.iter(|| super::exe(1_000_000));
    }

    // #[test]
    // fn test_exe_1000000000000() {
    //     assert_eq!(super::exe(1_000_000_000_000), 224427);
    // }

    // #[bench]
    // fn bench_exe_1000000000000(b: &mut Bencher) {
    //     b.iter(|| super::exe(100_0000_000_000));
    // }
}
