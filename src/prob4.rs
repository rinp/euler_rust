extern crate test;

// 全部繋げるならこれだが
// pub fn exe()->u32{
//     (100..1000).filter_map(|x| (x..1000).rev().map(|y|x*y).filter(|&x|is_palindrome(x)).take(1).next()).max().unwrap()
// }

pub fn exe()->u32{
    let mut palindrome = 0;
    for x in (100..1000).rev() {
        for y in (x..1000).rev(){
            let z = x*y;

            // ここで大幅に処理量を減らすことができる。
            if z <= palindrome {
                break;
            }
            if is_palindrome(z) {
                palindrome = z;
            }
        }
    }
    palindrome
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


    #[bench]
    fn bench_is_palindrome(b: &mut Bencher) {
        b.iter(||super::is_palindrome(123454321));
    }

    #[test]
    fn test_exe(){
        assert_eq!(super::exe(),906609);
    }
    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe());
    }
}
