use std::ops::{Neg, Range};
use num::{Num, NumCast};
use crate::groups::*;

pub struct IntegerGroup<Element: Num + Ord + Copy + NumCast + Neg<Output = Element>, BinaryOperator> {
  pub set: Range<Element>,
  pub identity: Element,
  pub binary_operator: BinaryOperator,
}

impl<Element: Num + Ord + Copy + NumCast + Neg<Output = Element>, BinaryOperator> GroupTraits for IntegerGroup<Element, BinaryOperator> {
  type T = Element;

  fn bin_op(&self, a: Self::T, b: Self::T) -> Self::T {
    a.add(b)
  }

  fn identity(&self) -> Element {
    self.identity
  }

  fn inverse(&self, a: Element) -> Element {
      a.neg()
  }

  fn associative(&self, a: Element, b: Element, c: Element) -> bool {
    self.bin_op(self.bin_op(a, b), c) == self.bin_op(a, self.bin_op(b, c))
  }
}

impl<Element: Num + Ord + Copy + NumCast + Neg<Output = Element>, BinaryOperator> AbelianGroupTraits for IntegerGroup<Element, BinaryOperator> {
  fn commutative(&self, a: Element, b: Element) -> bool {
    self.bin_op(a, b) == self.bin_op(a, b)
  }
}

impl<Element: Num + Ord + Copy + NumCast + Neg<Output = Element>, BinaryOperator> IntegerGroup<Element, BinaryOperator> {
  pub fn new(set: Range<Element>, identity: Element, binary_operator: BinaryOperator) -> Self {
    Self {
      set,
      identity,
      binary_operator,
    }
  }

  pub fn elements<'a>(&'a self) -> &'a Range<Element> {
    &self.set
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ints_20() {
        let nums = -20..20;
        let group = IntegerGroup::new(nums,
          0,
          |a : i32, b: i32| a + b);
        assert_eq!(group.bin_op(1, 2), 3);
        assert!(group.associative(1, 2, 3));
        assert!(group.commutative(1, 2));
        group.elements().clone().for_each(|i| {
          assert_eq!(group.inverse(i), -i);
          assert_eq!(group.bin_op(i, group.inverse(i)), group.identity());
        });
    }
}
