
pub fn exe(max: usize) -> usize {
    (1..).map(|x| fib(x)).take_while(|&x| x < max).filter(|x| x % 2 == 0).fold(0, |sum, x| sum + x)
}

fn fib(i: usize) -> usize {
    let result = match i {
        1 => 1,
        2 => 1,
        _ => fib_loop(i - 3, 1, 1),
    };
    result
}

fn fib_loop(a: usize, b: usize, c: usize) -> usize {
    match a {
        0 => b + c,
        _ => fib_loop(a - 1, b + c, b),
    }
}

#[cfg(test)]
mod tests {

    use answer;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_fib1() {
        assert_eq!(1, super::fib(1));
    }
    #[test]
    fn test_fib2() {
        assert_eq!(1, super::fib(2));
    }

    #[test]
    fn test_fib3() {
        assert_eq!(2, super::fib(3));
    }

    #[test]
    fn test_fib4() {
        assert_eq!(3, super::fib(4));
    }

    #[test]
    fn test_exe() {
        assert_eq!(answer::ANSWER_2, exe(4_000_000));
    }

    #[bench]
    fn bench_fib_30(b: &mut Bencher) {
        b.iter(|| super::fib(30));
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| exe(4_000_000));
    }
}
