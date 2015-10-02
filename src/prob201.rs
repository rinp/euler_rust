use std::collections::HashMap;

pub fn exe(array:Vec<u32>,n:u32)->u32{
    let mut map:HashMap<u64,Vec<u32>> = HashMap::new();
    let m = calc(map,n);
    let c:u32 = m.iter().filter(|&(k,v)|v.contains(&1)).count() as u32;
    let s:u32 = array.iter().sum();
    s*c
}

fn calc(map:HashMap<u64,Vec<u32>>,max:u32)->HashMap<u64,Vec<u32>>{
    match map.len() {
        max=> map,
        _=>{
            let mut new_map:HashMap<u64,Vec<u32>> = HashMap::new();

            map.iter().fold((),|_,(k,v)|{
                let m = v.iter().max().unwrap();
                ((m+1u32)..100).fold((),|_,x|{
                    v.insert(x);
                    new_map.insert(*k +x as u64,*v);
                });
            });
            calc(new_map,max)
        }
    }
}

#[cfg(test)]
mod tests{
    use test::Bencher;
    use answer;

    #[test]
    fn test_exe(){
        //U(B,3) = {10,12,14,18,21,25,27,29}であり, sum(U(B,3)) = 156
        assert_eq!(super::exe(vec![1u32,3,6,8,10,11],3),156);
    }

    #[test]
    fn test_exe2(){
        let v = (1u32..101).map(|x|x.pow(2)).collect();
        assert_eq!(super::exe(v,50),answer::ANSWER_201);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        let v = (1u32..101).map(|x|x.pow(2)).collect();
        b.iter(||super::exe(v,50));
    }
}
