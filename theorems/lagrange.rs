use std::collections::HashSet;
use std::hash::Hash;

pub fn is_subgroup<Element>(group: &HashSet<Element>, subset: &HashSet<Element>) -> bool where Element: Eq, Element: Hash {
  subset.intersection(group).count() == subset.len()
}

pub fn lagrange_theorem<Element>(group: &HashSet<Element>, subset: &HashSet<Element>) -> bool where Element: Eq, Element: Hash {
  if !is_subgroup(group, subset) {
    return false;
  }
  group.len() % subset.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lagrange_theorem() {
        let group: HashSet<i32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        let subset: HashSet<i32> = vec![0, 2, 4].into_iter().collect();
        assert!(lagrange_theorem(&group, &subset));
    }
}