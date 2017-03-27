

// #[cfg(test)]
// mod tests {
//     use test::Bencher;
//     use answer;
//     use std::collections::HashSet;

//     #[test]
//     fn conbination_add_1() {
//         let it = super::combination_add(4, 2);
//         let vecs: Vec<Vec<usize>> = it.collect();
//         assert_eq!(vecs.len(), 6);
//     }
//     #[test]
//     fn conbination_add_2() {
//         let it = super::combination_add(4, 2);
//         let vecs: HashSet<Vec<usize>> = it.collect();
//         let result: HashSet<Vec<usize>> = [vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2],
//                                            vec![1, 3], vec![2, 3]]
//             .iter()
//             .cloned()
//             .collect();
//         assert_eq!(vecs, result);

//     }

//     #[test]
//     fn conbination_1() {
//         let it = super::combination(vec![1, 5, 7, 8, 9], 3);
//         let vecs: HashSet<Vec<usize>> = it.collect();
//         let result: HashSet<Vec<usize>> = [vec![1, 5, 7],
//                                            vec![1, 5, 8],
//                                            vec![1, 5, 9],
//                                            vec![1, 7, 8],
//                                            vec![1, 7, 9],
//                                            vec![1, 8, 9],
//                                            vec![5, 7, 8],
//                                            vec![5, 7, 9],
//                                            vec![5, 8, 9],
//                                            vec![7, 8, 9]]
//             .iter()
//             .cloned()
//             .collect();
//         assert_eq!(vecs, result);

//     }

//     #[bench]
//     fn combination_add_bench(b: &mut Bencher) {
//         b.iter(|| super::combination_add(100, 50));
//     }

//     // #[test]
//     // fn test_exe() {
//     //     //U(B,3) = {10,12,14,18,21,25,27,29}であり, sum(U(B,3)) = 156
//     //     assert_eq!(super::exe(vec![1, 3, 6, 8, 10, 11], 3), 156);
//     // }

//     // #[test]
//     // fn test_exe2() {
//     //     let v: Vec<usize> = (1..101).map(|x: usize| x.pow(2u32)).collect();
//     //     assert_eq!(super::exe(v, 50), answer::ANSWER_201);
//     // }

//     // #[bench]
//     // fn bench_exe(b: &mut Bencher) {
//     //     let v:Vec<u32> = (1u32..101).map(|x|x.pow(2)).collect::<Vec<u32>>();
//     //     b.iter(||super::exe(v.clone(),50));
//     // }
// }
