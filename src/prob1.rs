extern crate test;

pub fn exe(max:u32)->(u32) {
    (1..).take_while(|&x|x<max).filter(|&x|divide_3_or_5(x)).fold(0, |sum, i| sum + i)
}
fn divide_3_or_5(i:u32)->bool{
    match i{
        _ if i%3==0 => true,
        _ if i%5==0 => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use prob2::test::Bencher;

    #[test]
    fn test_divide_3_or_5_in_3() {
        assert_eq!(true, super::divide_3_or_5(3));
    }
    #[test]
    fn test_divide_3_or_5_in_5() {
        assert_eq!(true, super::divide_3_or_5(5));
    }
    #[test]
    fn test_divide_3_or_5_in_2() {
        assert_eq!(false, super::divide_3_or_5(2));
    }

    #[test]
    fn test_exe1() {
        assert_eq!(233168, exe(1_000u32));
    }
    #[test]
    fn test_exe2() {
        assert_eq!(23, exe(10u32));
    }
    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||exe(1_000u32));
    }
}
