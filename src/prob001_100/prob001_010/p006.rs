pub fn exe(max: usize) -> usize {
    let a: usize = (1..max + 1).sum();
    let b: usize = (1..max + 1).map(|x| x * x).sum();
    a * a - b
}

pub fn exe2(max: usize) -> usize {
    max * (max + 1) * (3 * max * max - max - 2) / 12
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use answer;

    #[test]
    fn test_exe() {
        assert_eq!(super::exe2(10), 2640);
    }

    #[test]
    fn test_exe2() {
        assert_eq!(super::exe2(100), answer::ANSWER_6);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| super::exe(100));
    }
    #[bench]
    fn bench_exe2(b: &mut Bencher) {
        b.iter(|| super::exe2(100));
    }

}
