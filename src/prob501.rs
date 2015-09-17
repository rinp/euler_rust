extern crate test;

pub fn exe(max:u32)->u32{
    let list = prime_list(max);

    three_prime_comb(max,&list)+two_prime_comb(max,&list)+one_prime_comb(max,&list)

}

fn is_prime(i:u32)->bool {
    //30-> 6まででいい
    //2 3 4 5 6 7
    (2..i).take_while(|n|n.pow(2)<=i).all(|n|i % n != 0)
}

fn prime_list(max:u32)->Vec<u32>{
    let actual: Vec<_> = (2..max+1).filter(|&n|is_prime(n)).collect();
    actual
}

fn three_prime_comb(max:u32,list:&Vec<u32>)->u32{
    let mut count = 0;
    for p1 in list.iter() {
        if p1*p1*p1 > max {
            break;
        }
        for p2 in list.iter().filter(|&n| n > p1) {
            if p1*p2*p2 > max {
                break;
            }
            for p3 in list.iter().filter(|&n|n>p2) {
                if p1*p2*p3 > max {
                    break;
                }
                count+=1;
            }
        }
    }
    count
}

fn two_prime_comb(max:u32,list:&Vec<u32>)->u32{
    let mut count=0;
    for p1 in list.iter() {
        if p1*p1*p1 > max {
            break;
        }
        for p2 in list.iter().filter(|&n| n != p1) {
            if p1*p1*p1*p2 > max {
                break;
            }
            count+=1;
        }
    }
    count
}

fn one_prime_comb(max:u32,list:&Vec<u32>)->u32{
    let mut count=0;
    for p1 in list.iter() {
        if p1*p1*p1*p1*p1*p1*p1>max {
            break;
        }
        count+=1;
    }
    count
}

#[cfg(test)]
mod tests{
    use super::test::Bencher;
    #[test]
    fn test_is_prime(){
        assert_eq!(super::is_prime(4),false);
    }
    #[test]
    fn test_is_prime2(){
        assert_eq!(super::is_prime(19),true);
    }
    #[test]
    fn test_is_prime3(){
        assert_eq!(super::is_prime(3),true);
    }
    #[test]
    fn test_is_prime4(){
        assert_eq!(super::is_prime(2),true);
    }
    #[test]
    fn test_prime_list(){
        assert_eq!(super::prime_list(10),[2,3,5,7]);
    }
    #[test]
    fn test_prime_list2(){
        assert_eq!(super::prime_list(20),[2, 3, 5, 7, 11, 13, 17, 19]);
    }
    #[test]
    fn test_three_prime_comb(){
        let l = vec![2, 3, 5, 7];
        assert_eq!(super::three_prime_comb(147,&l),4);
    }
    #[test]
    fn test_exe1(){
        assert_eq!(super::exe(100),10);
    }
    #[test]
    fn test_exe2(){
        assert_eq!(super::exe(1000),180);
    }
    #[test]
    fn test_exe3(){
        assert_eq!(super::exe(1000000),224427);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe(1000000));
    }

    #[bench]
    fn bench_prime_list(b: &mut Bencher) {
        b.iter(||super::prime_list(1000000));
    }
    #[bench]
    fn bench_three_prime_comb(b: &mut Bencher) {
        let list = super::prime_list(1000000);
        b.iter(||super::three_prime_comb(1000000,&list));
    }
    #[bench]
    fn bench_two_prime_comb(b: &mut Bencher) {
        let list = super::prime_list(1000000);
        b.iter(||super::two_prime_comb(1000000,&list));
    }
    #[bench]
    fn bench_one_prime_comb(b: &mut Bencher) {
        let list = super::prime_list(1000000);
        b.iter(||super::one_prime_comb(1000000,&list));
    }
}
