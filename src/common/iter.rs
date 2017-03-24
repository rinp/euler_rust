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