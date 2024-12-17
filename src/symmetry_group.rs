use crate::groups::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum S3 { // Initial order is RGB
  E,   // RGB -> RGB or ()
  A,   // RGB -> GRB or (RG)
  B,   // RGB -> RBG or (GB)
  AB,  // RGB -> BRG or (RGB)
  BA,  // RGB -> GBR or (RBG)
  ABA, // RGB -> BGR or (RB)
}

pub struct SymmetryGroup<BinaryOperator> {
  pub set: Vec<S3>,
  pub identity: S3,
  pub binary_operator: BinaryOperator,
}

impl<BinaryOperator> GroupTraits for SymmetryGroup<BinaryOperator> {
  type T = S3;

  fn bin_op(&self, a: S3, b: S3) -> S3 {
    todo!("Implement bin_op for S3");
  }

  fn identity(&self) -> S3 {
    S3::E
  }

  fn inverse(&self, a: S3) -> S3 {
    let inverse = match a {
      S3::E => S3::E,
      S3::A => S3::A,
      S3::B => S3::B,
      S3::AB => S3::BA,
      S3::BA => S3::AB,
      S3::ABA => S3::ABA,
    };
    inverse
  }

  fn associative(&self, a: S3, b: S3, c: S3) -> bool {
    self.bin_op(self.bin_op(a, b), c) == self.bin_op(a, self.bin_op(b, c))
  }
}

impl<BinaryOperator> SymmetryGroup<BinaryOperator> {
  pub fn new(binary_operator: BinaryOperator) -> Self {
    Self {
      set : vec![S3::E, S3::A, S3::B, S3::AB, S3::BA, S3::ABA],
      identity : S3::E,
      binary_operator,
    }
  }

  pub fn elements<'a>(&'a self) -> &'a Vec<S3> {
    &self.set
  }

  pub fn order(&self) -> usize {
    self.set.len()
  }
}