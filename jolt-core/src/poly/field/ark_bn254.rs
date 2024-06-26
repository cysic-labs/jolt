use ark_ff::PrimeField;
use ark_std::UniformRand;

use super::JoltField;
impl JoltField for ark_bn254::Fr {
    const NUM_BYTES: usize = 32;

    fn random<R: rand_core::RngCore>(rng: &mut R) -> Self {
        <Self as UniformRand>::rand(rng)
    }

    fn is_zero(&self) -> bool {
        <Self as ark_std::Zero>::is_zero(self)
    }

    fn is_one(&self) -> bool {
        <Self as ark_std::One>::is_one(self)
    }

    fn zero() -> Self {
        <Self as ark_std::Zero>::zero()
    }

    fn one() -> Self {
        <Self as ark_std::One>::one()
    }

    fn from_u64(n: u64) -> Option<Self> {
        <Self as ark_ff::PrimeField>::from_u64(n)
    }

    fn square(&self) -> Self {
        <Self as ark_ff::Field>::square(self)
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), Self::NUM_BYTES);
        ark_bn254::Fr::from_le_bytes_mod_order(bytes)
    }
}
