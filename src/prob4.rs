extern crate test;

pub fn exe()->u32{
    (100..1000).filter_map(|x| (x..1000).map(|y|x*y).filter(|&x|is_palindrome(x)).max()).max().unwrap()
}

fn is_palindrome(num:u32)->bool{
    let str = num.to_string();
    str.chars().zip(str.chars().rev()).all(|(a,b)|a==b)
}

#[cfg(test)]
mod tests{
    use super::test::Bencher;

    #[test]
    fn test_is_palindrome1(){
        assert_eq!(super::is_palindrome(123321),true);
    }
    #[test]
    fn test_is_palindrome2(){
        assert_eq!(super::is_palindrome(000100),false);
    }
    #[test]
    fn test_is_palindrome3(){
        assert_eq!(super::is_palindrome(1234321),true);
    }

    #[test]
    fn test_exe(){
        assert_eq!(super::exe(),906609);
    }

    #[bench]
    fn bench_is_palindrome(b: &mut Bencher) {
        b.iter(||super::is_palindrome(123454321));
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe());
    }
}
