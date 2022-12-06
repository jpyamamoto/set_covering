use std::collections::HashSet;

pub type Set<T> = HashSet<T>;
pub type Subsets<T> = Vec<HashSet<T>>;
pub type Solution<T> = Subsets<T>;
