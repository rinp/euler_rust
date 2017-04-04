use std::collections::HashMap;

pub fn prime_iter() -> Box<Iterator<Item = u64>> {
    let from_two = (1u64..)
        .into_iter()
        .map(|x| if x % 2 == 0 { 3 * x + 1 } else { 3 * x + 2 })
        .scan(HashMap::new(), |map, i| {

            let (not_prime, base) =
                map.remove(&i).map(|x| (true, x)).unwrap_or_else(|| (false, &i * 2));

            let new_key: u64 = (1..)
                .map(|x| x * base + i)
                .filter(|x| x % 3 != 0)
                .filter(|x| !map.contains_key(x))
                .next()
                .unwrap();
            map.insert(new_key, base);

            let op: Option<u64> = if !not_prime { Some(i) } else { None };

            if map.len() % 50000 == 0 {
                println!("{}", i);
            }

            Some(op)

        })
        .filter_map(|s| s);

    let it = (2u64..4).chain(from_two);

    Box::new(it)

}


pub fn combination_iter<E: Clone + 'static>(vec: Vec<E>,
                                            comb: usize)
                                            -> Box<Iterator<Item = Vec<E>>> {
    let it = combination_add(vec.len(), comb);

    let n = it.map(move |i: Vec<usize>| {
        let vc: Vec<E> = vec.clone();
        let v: Vec<E> = i.into_iter()
            .map(|j: usize| vc[j].clone())
            .collect();
        v
    });
    Box::new(n)
}

fn combination_add(length: usize, max: usize) -> Box<Iterator<Item = Vec<usize>>> {
    let base_it = (0..length).into_iter().map(|x| vec![x]);

    let it: Box<Iterator<Item = Vec<usize>>> = Box::new(base_it);
    combination_add_in(length, it, 1, max)
}

fn combination_add_in(length: usize,
                      now: Box<Iterator<Item = Vec<usize>>>,
                      size: usize,
                      max: usize)
                      -> Box<Iterator<Item = Vec<usize>>> {
    if size == max {
        now
    } else {
        let it = now.flat_map(move |v| {
            (1 + *v.last().unwrap()..length)
                .into_iter()
                .map(move |n| {
                    let mut new_vec = v.to_vec();
                    new_vec.push(n);
                    new_vec
                })
        });
        combination_add_in(length, Box::new(it), size + 1, max)
    }

}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use std::collections::HashSet;

    #[test]
    fn prime_iter_test() {
        let v: Vec<u64> = super::prime_iter().take(5).collect();
        assert_eq!(v, [2, 3, 5, 7, 11]);

    }
    #[test]
    fn prime_iter_test_2() {
        let v: Vec<u64> = super::prime_iter().take(10).collect();
        assert_eq!(v, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

    }

    #[test]
    fn prime_iter_test_3() {
        let v: Vec<u64> = super::prime_iter().take(100).collect();
        let result: Vec<u64> =
            [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
             83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
             173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263,
             269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367,
             373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
             467, 479, 487, 491, 499, 503, 509, 521, 523, 541]
                .to_vec();
        assert_eq!(v, result);

    }
    // test common::iter::tests::prime_iter_bench_10                   ... bench:       1,991 ns/iter (+/- 151)
    // test common::iter::tests::prime_iter_bench_100                  ... bench:      45,856 ns/iter (+/- 1,394)
    // test common::iter::tests::prime_iter_bench_1000                 ... bench:     835,137 ns/iter (+/- 38,032)
    // test common::iter::tests::prime_iter_bench_10000                ... bench:  11,715,948 ns/iter (+/- 1,706,147)
    // test common::iter::tests::prime_iter_bench_100000               ... bench: 160,553,689 ns/iter (+/- 23,185,862)

    #[bench]
    fn prime_iter_bench_10(b: &mut Bencher) {
        b.iter(|| super::prime_iter().take(10).last());
    }
    #[bench]
    fn prime_iter_bench_100(b: &mut Bencher) {
        b.iter(|| super::prime_iter().take(100).last());
    }
    #[bench]
    fn prime_iter_bench_1000(b: &mut Bencher) {
        b.iter(|| super::prime_iter().take(1_000).last());
    }
    #[bench]
    fn prime_iter_bench_10000(b: &mut Bencher) {
        b.iter(|| super::prime_iter().take(10_000).last());
    }
    #[bench]
    fn prime_iter_bench_100000(b: &mut Bencher) {
        b.iter(|| super::prime_iter().take(100_000).last());
    }
    #[test]
    fn combination_add_1() {
        let it = super::combination_add(4, 2);
        let vecs: Vec<Vec<usize>> = it.collect();
        assert_eq!(vecs.len(), 6);
    }
    #[test]
    fn combination_add_2() {
        let it = super::combination_add(4, 2);
        let vecs: HashSet<Vec<usize>> = it.collect();
        let result: HashSet<Vec<usize>> = [vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2],
                                           vec![1, 3], vec![2, 3]]
            .iter()
            .cloned()
            .collect();
        assert_eq!(vecs, result);

    }

    #[test]
    fn combination_1() {
        let it = super::combination_iter(vec![1, 5, 7, 8, 9], 3);
        let vecs: HashSet<Vec<usize>> = it.collect();
        let result: HashSet<Vec<usize>> = [vec![1, 5, 7],
                                           vec![1, 5, 8],
                                           vec![1, 5, 9],
                                           vec![1, 7, 8],
                                           vec![1, 7, 9],
                                           vec![1, 8, 9],
                                           vec![5, 7, 8],
                                           vec![5, 7, 9],
                                           vec![5, 8, 9],
                                           vec![7, 8, 9]]
            .iter()
            .cloned()
            .collect();
        assert_eq!(vecs, result);

    }

    #[bench]
    fn combination_add_4_2(b: &mut Bencher) {
        b.iter(|| super::combination_add(4, 2).last());
    }
    #[bench]
    fn combination_add_10_2(b: &mut Bencher) {
        b.iter(|| super::combination_add(10, 2).last());
    }
    #[bench]
    fn combination_add_5_4(b: &mut Bencher) {
        b.iter(|| super::combination_add(5, 4).last());
    }
    #[bench]
    fn combination_add_10_5(b: &mut Bencher) {
        b.iter(|| super::combination_add(10, 5).last());
    }

    #[bench]
    fn combination_add_c_1_1(b: &mut Bencher) {
        b.iter(|| super::combination_add(20, 10).last());
    }

    #[bench]
    fn combination_add_c_1_2(b: &mut Bencher) {
        b.iter(|| super::combination_add(100, 3).last());
    }

    #[bench]
    fn combination_add_c_1_3(b: &mut Bencher) {
        b.iter(|| super::combination_add(100, 50).last());
    }

}
