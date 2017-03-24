fn main() {

    for a in 1..101 {
        for b in 1..101 {
            match b {
                _ if a == b => continue,
                _ => (),
            }
            for c in 1..101 {
                match c {
                    _ if c == a => continue,
                    _ if c == b => continue,
                    _ => (),
                }
                for d in 1..101 {
                    match d {
                        _ if d == a => continue,
                        _ if d == b => continue,
                        _ if d == c => continue,
                        _ => (),
                    }
                    for e in 1..101 {
                        match e {
                            _ if e == a => continue,
                            _ if e == b => continue,
                            _ if e == c => continue,
                            _ if e == d => continue,
                            _ => (),
                        }
                        for f in 1..101 {
                            match f {
                                _ if f == a => continue,
                                _ if f == b => continue,
                                _ if f == c => continue,
                                _ if f == d => continue,
                                _ if f == e => continue,
                                _ => (),
                            }

                            let right = a * a + b * b + c * c;
                            let left = d * d + e * e + f * f;

                            if right == left {
                                println!("{} {} {} = {} {} {}", a, b, c, d, e, f);
                            }

                        }
                    }
                }
            }
        }
    }
    println!("何か見つかりましたか？");
}