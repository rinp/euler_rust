// use std::thread;
// use std::sync::Arc;
// use std::sync::mpsc;
// use std::collections::HashMap;
// use rayon::prelude::*;
use std::cmp;
pub fn exe(max: u64) -> u64 {
    let split: usize = cmp::min(987654321, max) as usize;
    let len: usize = (max / split as u64) as usize;

    let mut count: u64 = 0;
    for a in 0..len {
        let mut v: Vec<u8> = vec![0u8;split];

        let l_min: u64 = (a * split + 1) as u64;
        let l_max: u64 = ((a + 1) * split) as u64;
        println!("{:12}～{:12}の検証を開始", l_min, l_max);
        for b in 2..(l_max / 2) + 1 {
            if b % (split / 20) as u64 == 0u64 {
                println!("{}", b);
            }
            //            println!("b={}", b);
            (2u64..)
                .map(|c| c * b)
                .skip_while(|&c| c < l_min)
                .take_while(|&c| c <= l_max)
                .fold((), |_, c| {
                    //l_min l_min+1  l_min+2... l_max
                    let d: usize = (c - l_min) as usize;
                    if v[d] < 7u8 {
                        //                        println!("add: {}->{} ({} _{})", c, b, v[d], d);
                        v[d] += 1;
                    }

                });
        }


        //      println!("{}～{}", l_min, l_max);
        count += v.iter().filter(|&x| x == &6u8).count() as u64;
    }

    count
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
    fn bench_exe_1_000(b: &mut Bencher) {
        b.iter(|| super::exe(1_000));
    }
    //bench:   2,861,188 ns/iter (+/- 120,110)


    // 600,185,692 ns/iter (+/- 64,254,897)
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
