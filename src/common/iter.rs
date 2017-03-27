use std::collections::HashMap;

pub fn prime_iter() -> Box<Iterator<Item = usize>> {
    let from_two = (1..)
        .into_iter()
        .map(|x| x * 2 + 1)
        .scan(HashMap::new(), |map, i| {
            let not_prime = map.contains_key(&i);
            let base: usize = if not_prime {
                map.remove(&i).unwrap()
            } else {
                i * 2
            };

            let new_key: usize =
                (1..).map(|x| x * base + i).filter(|x| !map.contains_key(x)).next().unwrap();
            map.insert(new_key, base);

            let op: Option<usize> = if !not_prime { Some(i) } else { None };

            Some(op)

        })
        .filter_map(|s| s);

    let it = (2..3).chain(from_two);

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

    let mut it: Box<Iterator<Item = Vec<usize>>> =
        Box::new((0..length).into_iter().map(|x| vec![x]));

    for i in 0..length {
        let max_clone = max.clone();
        let i_clone = i.clone();
        it = Box::new(it.flat_map(move |x: Vec<usize>| {

            if &i_clone <= x.last().unwrap() || x.len() == max_clone {

                return vec![x].into_iter();
            }

            let mut v: Vec<Vec<usize>> = Vec::new();

            let mut clone: Vec<usize> = x.iter().cloned().collect();
            v.push(x);
            clone.push(i_clone);
            v.push(clone);
            return v.into_iter();

        }));
    }

    return Box::new(it.filter(move |x| {
        let len = x.len();
        len.eq(&max)
    }));
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use std::collections::HashSet;

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
    fn combination_add_bench(b: &mut Bencher) {
        b.iter(|| super::combination_add(100, 50).last());
    }
}
