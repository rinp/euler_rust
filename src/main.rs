extern crate euler;
// use euler::common;
// use std::collections::HashMap;
use euler::prob501_600::prob501_510::p501;
fn main() {
    println!("{}", p501());
}
//     let max = 1000000000000;
//     let v_len = 4000000;
//     let base = vec![0u8;v_len];
//     let vs: Box<Iterator<Item = Vec<u8>>> =
//         Box::new((0..max / v_len).into_iter().map(|_| base.to_vec()));

//     let mut i: u64 = 1;
//     for a in vs {
//         for b in a.iter() {
//             println!("{}", i);
//             i += 1;
//         }
//     }

//     // let prime_max: u64 = max / 6;
//     // let prime_iter: Vec<u64> = common::iter::prime_iter().take_while(|i| i < &prime_max).collect();

//     // for a in prime_iter.iter() {
//     //     for b in prime_iter.iter().skip_while(|b| b <= &a) {
//     //         println!("{} {}", a, b);
//     //     }
//     // }
// }

fn p501() -> u64 {
    p501::exe(1_000_000_000_000)
}

// fn p201() {
//     let i_vec: Vec<usize> = (1..101).map(|x| x * x).collect();
//     let vec: Vec<(usize, Vec<usize>)> = common::iter::combination_iter(i_vec, 4)
//         .map(|v| {
//             let sum = v.iter().sum();
//             (sum, v)
//         })
//         .collect();

//     let vec3 = vec.to_vec();

//     let it2: Box<Iterator<Item = usize>> = Box::new(vec.iter().clone().map(|t: &(usize, _)| t.0));

//     let map = it2.fold(HashMap::new(), |mut map, sum: usize| {
//         if map.contains_key(&sum) {
//             map.insert(sum, false);
//         } else {
//             map.insert(sum, true);
//         }
//         map

//     });

//     let vec2: Vec<(usize, Vec<usize>)> = vec3.into_iter()
//         .filter(move |t| {
//             let sum = t.0;
//             map.get(&sum).unwrap().clone()
//         })
//         .collect();

//     for i in vec2.clone() {
//         println!("{:?}", i);
//     }
//     println!("len = {}", vec2.len());

// }