use common::iter;
pub fn exe(max: usize) -> usize {

    iter::prime_iter().map(|i| i as usize).nth(max - 1).unwrap()

}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use answer;

    #[test]
    fn test_exe_1() {
        assert_eq!(super::exe(1), 2);
    }

    #[test]
    fn test_exe_2() {
        assert_eq!(super::exe(2), 3);
    }

    #[test]
    fn test_exe_3() {
        assert_eq!(super::exe(3), 5);
    }

    #[test]
    fn test_exe() {
        assert_eq!(super::exe(6), 13);
    }

    #[test]
    fn test_exe2() {
        assert_eq!(super::exe(8), 19);
    }
    #[test]
    fn test_exe3() {
        assert_eq!(super::exe(10_001), answer::ANSWER_7);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| super::exe(10_001));
    }

}
