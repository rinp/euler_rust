pub fn exe(x: u64) -> usize {
    search_largest_prime(2, x)
}

fn search_largest_prime(mut x: u64, y: u64) -> usize {
    loop {
        if x >= y {
            return x as usize;
        }

        if y % x == 0 {
            assert!(x < y / x);
            return search_largest_prime(x, y / x);
        }

        x += 1;
    }
}

#[cfg(test)]
mod tests {
    use answer;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_divide() {
        assert_eq!(super::search_largest_prime(2, 10), 5);
    }
    #[test]
    fn test_divide2() {
        assert_eq!(super::search_largest_prime(2, 13195), 29);
    }

    #[test]
    fn test_exe() {
        assert_eq!(answer::ANSWER_3, exe(600_851_475_143));
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| exe(600_851_475_143));
    }
}
