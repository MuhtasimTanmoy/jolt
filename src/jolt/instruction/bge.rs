use ark_ff::PrimeField;

use super::{slt::SLTInstruction, JoltInstruction};
use crate::{
  jolt::subtable::{
    eq::EQSubtable, eq_abs::EQABSSubtable, eq_msb::EQMSBSubtable, gt_msb::GTMSBSubtable,
    lt_abs::LTABSSubtable, ltu::LTUSubtable, LassoSubtable,
  },
  utils::instruction_utils::chunk_and_concatenate_operands,
};

#[derive(Copy, Clone, Default, Debug)]
pub struct BGEInstruction(pub i64, pub i64);

impl JoltInstruction for BGEInstruction {
  fn combine_lookups<F: PrimeField>(&self, vals: &[F], C: usize, M: usize) -> F {
    // 1 - LTS(x, y) =
    F::one() - SLTInstruction(self.0, self.1).combine_lookups(vals, C, M)
  }

  fn g_poly_degree(&self, C: usize) -> usize {
    C
  }

  fn subtables<F: PrimeField>(&self) -> Vec<Box<dyn LassoSubtable<F>>> {
    vec![
      Box::new(GTMSBSubtable::new()),
      Box::new(EQMSBSubtable::new()),
      Box::new(LTUSubtable::new()),
      Box::new(EQSubtable::new()),
      Box::new(LTABSSubtable::new()),
      Box::new(EQABSSubtable::new()),
    ]
  }

  fn to_indices(&self, C: usize, log_M: usize) -> Vec<usize> {
    chunk_and_concatenate_operands(self.0 as u64, self.1 as u64, C, log_M)
  }
}

#[cfg(test)]
mod test {
  use ark_curve25519::Fr;
  use ark_std::{test_rng, One};
  use rand_chacha::rand_core::RngCore;

  use crate::{jolt::instruction::JoltInstruction, jolt_instruction_test};

  use super::BGEInstruction;

  #[test]
  fn bge_instruction_e2e() {
    let mut rng = test_rng();
    const C: usize = 8;
    const M: usize = 1 << 16;

    for _ in 0..256 {
      let x = rng.next_u64() as i64;
      let y = rng.next_u64() as i64;
      jolt_instruction_test!(BGEInstruction(x, y), (x >= y).into());
    }
    for _ in 0..256 {
      let x = rng.next_u64() as i64;
      jolt_instruction_test!(BGEInstruction(x, x), Fr::one());
    }
  }
}