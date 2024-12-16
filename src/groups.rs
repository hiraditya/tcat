//! Groups are a set with a binary operator that satisfies the following properties:
//! Identity Element
//! Inverse Element
//! Associative Property

pub trait GroupTraits {
  type T;
  fn bin_op(&self, a: Self::T, b: Self::T) -> Self::T;
  fn identity(&self) -> Self::T;
  fn inverse(&self, a: Self::T) -> Self::T;
  fn associative(&self, a: Self::T, b: Self::T, c: Self::T) -> bool;
}

pub trait AbelianGroupTraits : GroupTraits {
  fn commutative(&self, a: Self::T, b: Self::T) -> bool;
}
