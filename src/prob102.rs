pub fn exe(path:&str)->u32{
    use std::fs::File;
    use std::error::Error;
    use std::io::{BufReader,BufRead};
    use std::str::FromStr;

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open : {}  {}", Error::description(&why),why),
        Ok(file) => file,
    };

    let mut buf = BufReader::new(&file);
    for line in buf.lines() {
        let v:Vec<isize> = match line {
            Ok(l)=>l.split(',').map(|s|{
                        let n:isize = FromStr::from_str(s).unwrap();
                        n
                    }).collect(),
            Err(e)=>panic!("{}",e),
        };
        let t = Triangle::new(v);
        println!("{},{}",t.a.x,t.c.y);
    }
    3
}

struct Point {
    x:isize,
    y:isize
}

struct Triangle {
    a:Point,
    b:Point,
    c:Point,
}
impl Triangle{
    pub fn new(v:Vec<isize>)->Triangle{
        Triangle{a:Point{x:v[0],y:v[1]},b:Point{x:v[2],y:v[3]},c:Point{x:v[4],y:v[5]}}
    }
    pub fn has_origin(&self)->bool{
        let s1 = (self.a.x*self.b.y-self.a.y*self.b.x).abs();
        let s2 = (self.b.x*self.c.y-self.b.y*self.c.x).abs();
        let s3 = (self.c.x*self.a.y-self.c.y*self.a.x).abs();
        let s  = ((self.b.x-self.a.x)*(self.c.y-self.a.y)-(self.b.y-self.a.y)*(self.c.x-self.a.x)).abs();
        s1+s2+s3 ==s
    }
}

#[cfg(test)]
mod tests{
    use test::Bencher;
    use answer;

    #[test]
    fn test_exe(){
        assert_eq!(super::exe("./ref/p102_triangles_test.txt"),4/*1*/);
    }

    #[test]
    fn test_exe2(){
        assert_eq!(super::exe("./ref/p102_triangles.txt"),4/*answer::ANSWER_102*/);
    }

    #[test]
    fn test_match(){
        let t = super::Triangle([0,0,1,0,0,1]);
        assert_eq!(t.has_origin(), true);
    }

    #[bench]
    fn bench_exe(b: &mut Bencher) {
        b.iter(||super::exe("./ref/p102_triangles.txt"));
    }
}
