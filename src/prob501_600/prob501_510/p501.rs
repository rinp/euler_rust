use common::iter;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

fn prim_vec(max: &u64) -> Vec<u64> {
    let prime_max: u64 = max / 6;
    let primes: Vec<u64> = iter::prime_iter()
        .take_while(|i| i < &prime_max)
        //.inspect(|x| println!("{}", x))
        .collect();
    primes
}

pub fn exe(max: u64) -> u64 {
    let primes: Vec<u64> = prim_vec(&max);
    println!("素数生成完了:{}", primes.len());
    let arc_primes = Arc::new(primes);
    let (tx, rx) = mpsc::channel();

    let primes_1 = arc_primes.clone();
    let tx_1 = tx.clone();
    thread::spawn(move || {
        let count_1: u64 = primes_1.iter()
        .clone()
        .take_while(|&a| a * a * a * a * a * a * a < max)
//        .inspect(|x| println!("{}", x))
        .count() as u64;

        tx_1.send(count_1).unwrap();
    });

    let primes_2 = arc_primes.clone();
    let tx_2 = tx.clone();
    thread::spawn(move || {
        let count_2: u64 = primes_2.iter()
            .clone()
            .map(|&a: &u64| {
                println!("2素数:{}", a);

                primes_2.iter()
                    .clone()
                    .skip_while(|&b| b <= &a)
                    .take_while(|&b| a * a * a * b < max)
                    .count() as u64 +
                primes_2.iter()
                    .clone()
                    .skip_while(|&b| b <= &a)
                    .take_while(|&b| a * b * b * b < max)
                    .count() as u64
            })
            .sum::<u64>();

        tx_2.send(count_2).unwrap();
    });

    let primes_3 = arc_primes.clone();
    let tx_3 = tx.clone();
    thread::spawn(move || {
        let count_3: u64 = primes_3.iter()
            .clone()
            .map(|&a: &u64| {
                println!("3素数:{}", a);

                primes_3.iter()
                    .clone()
                    .skip_while(|&b| b <= &a)
                    .map(|&b: &u64| {
                        primes_3.iter()
                        .clone()
                        .skip_while(|&c| c <= &b)
                        .take_while(|&c| a * b * c < max)
                 //       .inspect(|c| println!("{} {} {}", a, b, c))
                        .count() as u64
                    })
                    .take_while(|&sum| 0 < sum)
                    .sum::<u64>()
            })
            .take_while(|&sum| 0 < sum)
            .sum();
        tx_3.send(count_3).unwrap();

    });

    rx.iter().take(3).sum()

}

#[cfg(test)]
mod tests {
    // use answer;
    use test::Bencher;

    #[test]
    fn test_exe_100() {
        assert_eq!(super::exe(100), 10);
    }

    #[bench]
    fn bench_prm_100(b: &mut Bencher) {
        b.iter(|| super::prim_vec(&100));
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
    #[bench]
    fn bench_prm_1000(b: &mut Bencher) {
        b.iter(|| super::prim_vec(&1_000));
    }

    #[test]
    fn test_exe_1000000() {
        assert_eq!(super::exe(1_000_000), 224427);
    }

    #[bench]
    fn bench_exe_1000000(b: &mut Bencher) {
        b.iter(|| super::exe(1_000_000));
    }

    #[bench]
    fn bench_prim_1000000(b: &mut Bencher) {
        b.iter(|| super::prim_vec(&1_000_000));
    }

    #[test]
    fn test_exe_1000000000000() {
        assert_eq!(super::exe(1_000_000_000_000), 224427);
    }

    // #[bench]
    // fn bench_exe_1000000000000(b: &mut Bencher) {
    //     b.iter(|| super::exe(100_0000_000_000));
    // }
}
