
pub fn exe() -> usize {
    let mut palindrome = 0;
    for x in (100..1000).rev() {
        for y in (x..1000).rev() {
            let z = x * y;

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

fn is_palindrome(num: usize) -> bool {
    let str = num.to_string();
    str.chars().eq(str.chars().rev())
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use answer;

    #[test]
    fn test_is_palindrome1() {
        assert_eq!(super::is_palindrome(123321), true);
    }
    #[test]
    fn test_is_palindrome2() {
        assert_eq!(super::is_palindrome(000100), false);
    }
    #[test]
    fn test_is_palindrome3() {
        assert_eq!(super::is_palindrome(1234321), true);
    }


    #[bench]
    fn bench_is_palindrome(b: &mut Bencher) {
        b.iter(|| super::is_palindrome(123454321));
    }

    #[test]
    fn test_exe() {
        assert_eq!(super::exe(), answer::ANSWER_4);
    }


    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| super::exe());
    }

}
