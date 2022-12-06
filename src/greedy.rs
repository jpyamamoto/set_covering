use std::hash::Hash;
use crate::definitions::{Set, Solution, Subsets};

pub fn greedy<T: Clone + Hash + Eq>(universe: Set<T>, collection: Subsets<T>) -> Solution<T> {
    let mut u: Set<T> = universe.clone();
    let mut solution: Solution<T> = vec![];

    while !u.is_empty() {
        let chosen_set: &Set<T> = collection.iter()
                                            .max_by_key(|s| s.intersection(&u).count())
                                            .unwrap();

        u = u.difference(chosen_set).cloned().collect::<>();
        solution.push(chosen_set.clone());
    }

    solution
}
