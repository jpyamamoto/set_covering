use common_macros::hash_set;
use definitions::{Set, Subsets, Solution};
use greedy::greedy;

pub mod definitions;
pub mod greedy;

fn main() {
    // Test case 1
    let universe1: Set<i32> = (1..=12).collect();
    let subsets1: Subsets<i32> = vec![hash_set!{1, 2, 3, 4, 5, 6},
                                      hash_set!{5, 6, 8, 9},
                                      hash_set!{1, 4, 7, 10},
                                      hash_set!{2, 5, 7, 8, 11},
                                      hash_set!{3, 6, 9, 12},
                                      hash_set!{10, 11}];
    println!("Test case 1.");
    test_case(universe1, subsets1);

    // Test case 2
    println!("--------------------------------------------------");
    let universe2: Set<i32> = (1..=12).collect();
    let subsets2: Subsets<i32> = vec![hash_set!{1, 2, 3, 4, 6, 7, 9},
                                      hash_set!{4, 5, 7, 8},
                                      hash_set!{1, 4},
                                      hash_set!{2, 5},
                                      hash_set!{3, 6},
                                      hash_set!{6, 11},
                                      hash_set!{7, 8, 9, 10, 11, 12}];
    println!("Test case 2.");
    test_case(universe2, subsets2);

    // Test case 3
    println!("--------------------------------------------------");
    let universe3: Set<i32> = (1..=14).collect();
    let subsets3: Subsets<i32> = vec![hash_set!{1, 2, 3, 4, 5, 6, 7},
                                      hash_set!{8, 9, 10, 11, 12, 13, 14},
                                      hash_set!{1, 8},
                                      hash_set!{2, 3, 9, 10},
                                      hash_set!{4, 5, 6, 7, 11, 12, 13, 14}];
    println!("Test case 3.");
    test_case(universe3, subsets3);

    // Test case 4
    println!("--------------------------------------------------");
    let universe4: Set<i32> = (1..=15).collect();
    let subsets4: Subsets<i32> = vec![hash_set!{1, 3, 4, 6, 7},
                                      hash_set!{4, 7, 8, 12},
                                      hash_set!{2, 5, 9, 11, 13},
                                      hash_set!{1, 2, 14, 15},
                                      hash_set!{3, 6, 10, 12, 14},
                                      hash_set!{8, 14, 15},
                                      hash_set!{1, 2, 6, 11},
                                      hash_set!{1, 2, 4, 6, 8, 12}];
    println!("Test case 4.");
    test_case(universe4, subsets4);

    // Test case 5
    println!("--------------------------------------------------");
    let universe5: Set<i32> = (1..=7).collect();
    let subsets5: Subsets<i32> = vec![hash_set!{3, 7},
                                      hash_set!{2, 4},
                                      hash_set!{3, 4, 5, 6},
                                      hash_set!{5},
                                      hash_set!{1},
                                      hash_set!{1, 2, 6, 7}];
    println!("Test case 5.");
    test_case(universe5, subsets5);
}

fn test_case(universe: Set<i32>, subsets: Subsets<i32>) {
    println!("  Input (X, F ⊆ P(X))");
    println!("    X = {:?}", universe);
    println!("    F = {:?}", subsets);

    let solution: Solution<i32> = greedy(universe, subsets);

    println!("  Solution C ⊆ F");
    println!("    C = {:?}", solution);
    println!("    |C| = {:?}", solution.len());
}
