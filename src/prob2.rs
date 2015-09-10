extern crate test;

pub fn exe(max:u32)->(u32) {
    (1..).map(|x| fib(x)).take_while(|&x|x<max).filter(|x|x%2==0).fold(0,|sum,x|sum+x)
}

fn fib(i:u32)->(u32){
    match i {
        1|2=>1,
        _ =>fib(i-1)+fib(i-2)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use prob2::test::Bencher;

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
        assert_eq!(4613732, exe(4_000_000u32));
    }

    #[bench]
    fn bench_fib_30(b: &mut Bencher) {
        b.iter(|| super::fib(30));
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||exe(4_000_000u32));
    }
}
