#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::convert::TryInto;

use halo2derive::impl_field;
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

impl_field!(
    bn256_scalar,
    Fr,
    modulus = "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
    mul_gen = "7",
    zeta = "30644e72e131a029048b6e193fd84104cc37a73fec2bc5e9b8ca0b2d36636f23",
    from_uniform = [64, 48],
    endian = "little",
);

crate::extend_field_legendre!(Fr);
crate::impl_binops_calls!(Fr);
crate::impl_binops_additive!(Fr, Fr);
crate::impl_binops_multiplicative!(Fr, Fr);
crate::field_bits!(Fr);
crate::serialize_deserialize_primefield!(Fr);

#[cfg(feature = "ark-serialize")]
mod arkworks_serialize {
    use super::Fr;
    use ark_serialize::{
        CanonicalDeserialize, CanonicalSerialize, Compress, Validate, SerializationError,
    };
    use halo2curves::ff::PrimeField; // or crate::ff::PrimeField depending on paths
    use subtle::CtOption;

    impl CanonicalSerialize for Fr {
        fn serialize_with_mode<W: ark_serialize::Write>(
            &self,
            mut writer: W,
            _compress: Compress,
        ) -> Result<(), SerializationError> {
            writer.write_all(&self.to_repr())?;
            Ok(())
        }

        fn serialized_size(&self, _compress: Compress) -> usize {
            32
        }
    }

    impl CanonicalDeserialize for Fr {
        fn deserialize_with_mode<R: ark_serialize::Read>(
            mut reader: R,
            _compress: Compress,
            _validate: Validate,
        ) -> Result<Self, SerializationError> {
            let mut bytes = [0u8; 32];
            reader.read_exact(&mut bytes)?;
            let ct: CtOption<Fr> = Fr::from_repr(bytes);
            ct.into_option().ok_or(SerializationError::InvalidData)
        }
    }
}

#[cfg(feature = "bn256-table")]
pub use table::FR_TABLE;
#[cfg(not(feature = "bn256-table"))]
crate::impl_from_u64!(Fr);
#[cfg(feature = "bn256-table")]
impl From<u64> for Fr {
    fn from(val: u64) -> Fr {
        if val < 65536 {
            FR_TABLE[val as usize]
        } else {
            Self([val, 0, 0, 0]) * Fr::R2
        }
    }
}
crate::impl_from_bool!(Fr);

#[cfg(feature = "bn256-table")]
#[rustfmt::skip]
mod table;
#[cfg(feature = "bn256-table")]
#[cfg(test)]
mod table_tests;

#[cfg(test)]
mod test {
    use super::Fr;
    use crate::{
        arith_test, constants_test, from_uniform_bytes_test, legendre_test, serde_test, test,
    };

    constants_test!(Fr);

    arith_test!(Fr);
    legendre_test!(Fr);
    test!(arith, Fr, sqrt_test, 1000);

    serde_test!(Fr PrimeFieldBits);
    from_uniform_bytes_test!(Fr, 1000, L 64, L 48);
}
