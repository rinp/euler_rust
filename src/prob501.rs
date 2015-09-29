// extern crate test;
//
// pub fn exe(max:u64)->u32 {
//     let mut table : Vec<u8> = Vec::new();
//     for i in (0..max+1){
//         table.push(0);
//     }
//
//     let mut count = 0;
//     for l in (2..max+1) {
//         let i = table[l as usize];
//         if i == 6 {
//             count+=1;
//             for l2 in (2 * l..).step_by(l).take_while(|&x|x<=max) {
//                 table[l2 as usize] =7;
//             }
//         } else if i > 6 {
//             continue;
//         } else {
//             for l2 in (2 * l..).step_by(l).take_while(|&x|x<=max) {
//                 if table[l2 as usize] < 7 {
//                     table[l2 as usize] = table[l2 as usize] + 1;
//                 }
//             }
//         }
//     }
//     count
// }
//
// #[cfg(test)]
// mod tests{
//     use super::test::Bencher;
//
//     #[test]
//     fn test_exe_100(){
//         assert_eq!(super::exe(100),10);
//     }
//
//     #[bench]
//     fn bench_exe_100(b: &mut Bencher) {
//         b.iter(||super::exe(100));
//     }
//
//     #[test]
//     fn test_exe_1000(){
//         assert_eq!(super::exe(1000),180);
//     }
//
//     #[bench]
//     fn bench_exe_1000(b: &mut Bencher) {
//         b.iter(||super::exe(1000));
//     }
//
//     #[test]
//     fn test_exe_1000000(){
//         assert_eq!(super::exe(1000000),224427);
//     }
//
//     #[bench]
//     fn bench_exe_1000000(b: &mut Bencher) {
//         b.iter(||super::exe(1000000));
//     }
//
//     #[test]
//     fn test_exe_1000000000000(){
//         assert_eq!(super::exe(1000000000000),224427);
//     }
//
//     #[bench]
//     fn bench_exe_1000000000000(b: &mut Bencher) {
//         b.iter(||super::exe(1000000000000));
//     }
// }
