pub fn exe(max: &usize) -> usize {
    (1..).take_while(|x| x < max).filter(|x| divide_3_or_5(x)).sum()
}

fn divide_3_or_5(i: &usize) -> bool {
    match i {
        _ if i % 3 == 0 => true,
        _ if i % 5 == 0 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use answer;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_divide_3_or_5_in_3() {
        assert_eq!(true, super::divide_3_or_5(&3));
    }
    #[test]
    fn test_divide_3_or_5_in_5() {
        assert_eq!(true, super::divide_3_or_5(&5));
    }
    #[test]
    fn test_divide_3_or_5_in_2() {
        assert_eq!(false, super::divide_3_or_5(&2));
    }

    #[test]
    fn test_exe1() {
        assert_eq!(exe(&1_000), answer::ANSWER_1);
    }

    #[test]
    fn test_exe2() {
        assert_eq!(exe(&10), 23);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| exe(&1_000));
    }
}
