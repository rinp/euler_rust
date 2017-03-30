extern crate euler;
use euler::common;
use std::collections::HashMap;
use euler::prob501_600::prob501_510::p501;
fn main() {
    p501(100);
}

fn p501(i: u64) {
    let _ = p501::exe(i);
}

fn p201() {
    let i_vec: Vec<usize> = (1..101).map(|x| x * x).collect();
    let vec: Vec<(usize, Vec<usize>)> = common::iter::combination_iter(i_vec, 4)
        .map(|v| {
            let sum = v.iter().sum();
            (sum, v)
        })
        .collect();

    let vec3 = vec.to_vec();

    let it2: Box<Iterator<Item = usize>> = Box::new(vec.iter().clone().map(|t: &(usize, _)| t.0));

    let map = it2.fold(HashMap::new(), |mut map, sum: usize| {
        if map.contains_key(&sum) {
            map.insert(sum, false);
        } else {
            map.insert(sum, true);
        }
        map

    });

    let vec2: Vec<(usize, Vec<usize>)> = vec3.into_iter()
        .filter(move |t| {
            let sum = t.0;
            map.get(&sum).unwrap().clone()
        })
        .collect();

    for i in vec2.clone() {
        println!("{:?}", i);
    }
    println!("len = {}", vec2.len());

}