use std::collections::HashMap;

pub fn exe(array:Vec<u32>,n:u32)->u32{
    let map:HashMap<u64,Vec<u32>> = HashMap::new();
    let m = calc(map,n as usize);
    let c:u32 = m.values().filter(|&v|v.contains(&1u32)).count() as u32;
    let s:u32 = array.iter().fold(0,|s,n|s+n);
    s*c
}

fn calc(map:HashMap<u64,Vec<u32>>,max:usize)->HashMap<u64,Vec<u32>>{

    match  map.len()  {
        x if x== max =>map,
        _=>{
            let mut new_map:HashMap<u64,Vec<u32>> = HashMap::new();

            map.iter().fold((),|_,(k,v)|{
                let m = v.iter().max().unwrap();
                ((m+1u32)..100).fold((),|_,x|{
                    let mut v2:Vec<u32> =v.clone();
                    v2.push(x);
                    new_map.insert(*k +x as u64,v2);
                });
            });
            calc(new_map,max)
        },
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
        let v:Vec<u32> = (1u32..101).map(|x|x.pow(2)).collect::<Vec<u32>>();
        assert_eq!(super::exe(v,50),answer::ANSWER_201);
    }

    // #[bench]
    // fn bench_exe(b: &mut Bencher) {
    //     let v:Vec<u32> = (1u32..101).map(|x|x.pow(2)).collect::<Vec<u32>>();
    //     b.iter(||super::exe(v.clone(),50));
    // }
}
