extern crate test;
use std::cmp::min;
pub fn exe(max:u32)->u32{
    (2..max+1).rev().fold(1,|sum,x|sum*x/greatest_common_divisor(sum,x))
}

fn greatest_common_divisor(a:u32,b:u32)->u32{
    (2..min(a,b)+1).rev().filter(|x|b%x==0&&a%x==0).take(1).next().unwrap_or(1)
}

#[cfg(test)]
mod tests{
    use super::test::Bencher;

    #[test]
    fn test_greatest_common_divisor1(){
        assert_eq!(super::greatest_common_divisor(4*3,4*35),4);
    }
    #[test]
    fn test_greatest_common_divisor2(){
        assert_eq!(super::greatest_common_divisor(1024,81),1);
    }
    #[test]
    fn test_greatest_common_divisor3(){
        assert_eq!(super::greatest_common_divisor(100*3,100*2),100);
    }

    #[test]
    fn test_exe(){
        assert_eq!(super::exe(10),2520);
    }

    #[test]
    fn test_exe2(){
        assert_eq!(super::exe(20),232792560);
    }

    #[bench]
    fn bench_is_palindrome(b: &mut Bencher) {
        b.iter(||super::greatest_common_divisor(100*3,100*2));
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe(20));
    }
}
