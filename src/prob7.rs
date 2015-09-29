pub fn exe(max:u32)->u32{
    match max {
        1=>2,
        2=>3,
        _=>{
            let mut number=3;
            let mut primes =vec![2,3];
            loop {
                if primes.iter().any(|&x|number%x==0){
                    number+=2;
                    continue;
                }
                primes.push(number);
                if primes.len() == max as usize {
                    return number;
                }
                number+=2;
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use test::Bencher;
    use answer;

    #[test]
    fn test_exe(){
        assert_eq!(super::exe(6),13);
    }

    #[test]
    fn test_exe2(){
        assert_eq!(super::exe(8),19);
    }
    #[test]
    fn test_exe3(){
        assert_eq!(super::exe(10_001),answer::ANSWER_7);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe(10_001));
    }
}
