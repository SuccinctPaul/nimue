use group::{ff::PrimeField, Group, GroupEncoding};

use crate::{
    plugins::{bytes_modp, bytes_uniform_modp},
    ByteIOPattern, DuplexHash, IOPattern,
};

use super::{FieldIOPattern, GroupIOPattern};

impl<F, H> FieldIOPattern<F> for IOPattern<H>
where
    F: PrimeField,
    H: DuplexHash,
{
    fn add_scalars(self, count: usize, label: &str) -> Self {
        self.add_bytes(count * bytes_modp(F::NUM_BITS), label)
    }

    fn challenge_scalars(self, count: usize, label: &str) -> Self {
        self.challenge_bytes(count * bytes_uniform_modp(F::NUM_BITS), label)
    }
}

impl<G, H> GroupIOPattern<G> for IOPattern<H>
where
    G: Group + GroupEncoding,
    G::Repr: AsRef<[u8]>,
    H: DuplexHash,
{
    fn add_points(self, count: usize, label: &str) -> Self {
        let n = G::Repr::default().as_ref().len();
        self.add_bytes(count * n, label)
    }
}
