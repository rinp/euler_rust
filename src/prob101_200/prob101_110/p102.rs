pub fn exe(path: &str) -> usize {
    use std::fs::File;
    use std::error::Error;
    use std::io::{BufReader, BufRead};
    use std::str::FromStr;

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open : {}  {}", Error::description(&why), why),
        Ok(file) => file,
    };

    let buf = BufReader::new(&file);
    buf.lines()
        .map(|line| {
            let v: Vec<isize> = match line {
                Ok(l) => {
                    l.split(',')
                        .map(|s| FromStr::from_str(s).unwrap())
                        .collect()
                }
                Err(e) => panic!("{}", e),
            };
            Triangle::new(v)
        })
        .filter(|t| t.has_origin())
        .count()
}

struct Point {
    x: isize,
    y: isize,
}

struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}
impl Triangle {
    pub fn new(v: Vec<isize>) -> Triangle {
        Triangle {
            a: Point { x: v[0], y: v[1] },
            b: Point { x: v[2], y: v[3] },
            c: Point { x: v[4], y: v[5] },
        }
    }
    pub fn has_origin(&self) -> bool {
        let s1 = (self.a.x * self.b.y - self.a.y * self.b.x).abs();
        let s2 = (self.b.x * self.c.y - self.b.y * self.c.x).abs();
        let s3 = (self.c.x * self.a.y - self.c.y * self.a.x).abs();
        let s = ((self.b.x - self.a.x) * (self.c.y - self.a.y) -
                 (self.b.y - self.a.y) * (self.c.x - self.a.x))
            .abs();
        s1 + s2 + s3 == s
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use answer;

    #[test]
    fn test_exe() {
        assert_eq!(super::exe("./data/p102_triangles_test.txt"), 1);
    }

    #[test]
    fn test_exe2() {
        assert_eq!(super::exe("./data/p102_triangles.txt"), answer::ANSWER_102);
    }

    #[test]
    fn test_has_origin() {
        let t = super::Triangle::new(vec![0, 0, 1, 0, 0, 1]);
        assert_eq!(t.has_origin(), true);
    }

    #[test]
    fn test_has_origin2() {
        let t = super::Triangle::new(vec![0, 0, 1, 0, 0, 1]);
        assert_eq!(t.has_origin(), true);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(|| super::exe("./data/p102_triangles.txt"));
    }
}
