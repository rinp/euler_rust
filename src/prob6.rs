extern crate test;

pub fn exe(max:u64)->u64{
    let a = (1..max+1).fold(0,|sum,x|sum+x);
    a*a-(1..max+1).map(|x|x*x).fold(0,|sum,x|sum+x)
}

#[cfg(test)]
mod tests{
    use super::test::Bencher;

    #[test]
    fn test_exe(){
        assert_eq!(super::exe(10),2640);
    }

    #[test]
    fn test_exe2(){
        assert_eq!(super::exe(100),25164150);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe(100));
    }
}
