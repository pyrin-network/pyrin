// use sha3::CShake256;
use once_cell::sync::Lazy;

pub type BlueWorkType = pyrin_math::Uint192;

pub trait HasherBase {
    fn update<A: AsRef<[u8]>>(&mut self, data: A) -> &mut Self;
}

pub trait Hasher: HasherBase + Clone + Default {
    fn finalize(self) -> crate::Blake2Hash;
    fn reset(&mut self);
    #[inline(always)]
    fn hash<A: AsRef<[u8]>>(data: A) -> crate::Blake2Hash {
        let mut hasher = Self::default();
        hasher.update(data);
        hasher.finalize()
    }
}

pub trait HasherExtensions {
    /// Writes the len as u64 little endian bytes
    fn write_len(&mut self, len: usize) -> &mut Self;

    /// Writes the boolean as a u8
    fn write_bool(&mut self, element: bool) -> &mut Self;

    /// Writes a single u8
    fn write_u8(&mut self, element: u8) -> &mut Self;

    /// Writes the u16 as a little endian u8 array
    fn write_u16(&mut self, element: u16) -> &mut Self;

    /// Writes the u32 as a little endian u8 array
    fn write_u32(&mut self, element: u32) -> &mut Self;

    /// Writes the u64 as a little endian u8 array
    fn write_u64(&mut self, element: u64) -> &mut Self;

    /// Writes blue work as big endian bytes w/o the leading zeros
    /// (emulates bigint.bytes() in the pyrind golang ref)
    fn write_blue_work(&mut self, work: BlueWorkType) -> &mut Self;

    /// Writes the number of bytes followed by the bytes themselves
    fn write_var_bytes(&mut self, bytes: &[u8]) -> &mut Self;

    /// Writes the array len followed by each element as [[u8]]
    fn write_var_array<D: AsRef<[u8]>>(&mut self, arr: &[D]) -> &mut Self;
}

/// Fails at compile time if `usize::MAX > u64::MAX`.
/// If `usize` will ever grow larger than `u64`, we need to verify
/// that the lossy conversion below at `write_len` remains precise.
const _: usize = u64::MAX as usize - usize::MAX;

impl<T: HasherBase> HasherExtensions for T {
    #[inline(always)]
    fn write_len(&mut self, len: usize) -> &mut Self {
        self.update((len as u64).to_le_bytes())
    }

    #[inline(always)]
    fn write_bool(&mut self, element: bool) -> &mut Self {
        self.update(if element { [1u8] } else { [0u8] })
    }

    fn write_u8(&mut self, element: u8) -> &mut Self {
        self.update(element.to_le_bytes())
    }

    fn write_u16(&mut self, element: u16) -> &mut Self {
        self.update(element.to_le_bytes())
    }

    #[inline(always)]
    fn write_u32(&mut self, element: u32) -> &mut Self {
        self.update(element.to_le_bytes())
    }

    #[inline(always)]
    fn write_u64(&mut self, element: u64) -> &mut Self {
        self.update(element.to_le_bytes())
    }

    #[inline(always)]
    fn write_blue_work(&mut self, work: BlueWorkType) -> &mut Self {
        let be_bytes = work.to_be_bytes();
        let start = be_bytes.iter().copied().position(|byte| byte != 0).unwrap_or(be_bytes.len());

        self.write_var_bytes(&be_bytes[start..])
    }

    #[inline(always)]
    fn write_var_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.write_len(bytes.len()).update(bytes)
    }

    #[inline(always)]
    fn write_var_array<D: AsRef<[u8]>>(&mut self, arr: &[D]) -> &mut Self {
        self.write_len(arr.len());
        for d in arr {
            self.update(d);
        }
        self
    }
}


// Implemented manually in pow_hashers:
//  struct PowHash => `cSHAKE256("ProofOfWorkHash")
//  struct KHeavyHash => `cSHAKE256("HeavyHash")
pub use crate::pow_hashers::{KHeavyHash, PowHash};
blake2b_hasher! {
    struct TransactionHash => b"TransactionHash",
    struct TransactionID => b"TransactionID",
    struct TransactionSigningHash => b"TransactionSigningHash",
    struct BlockHash => b"BlockHash",
    struct ProofOfWorkHash => b"ProofOfWorkHash",
    struct MerkleBranchHash => b"MerkleBranchHash",
    struct MuHashElementHash => b"MuHashElement",
    struct MuHashFinalizeHash => b"MuHashFinalize",
    struct PersonalMessageSigningHash => b"PersonalMessageSigningHash",
    struct ContractAddress => b"ContractAddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
}

sha256_hasher! {
    struct TransactionSigningHashECDSA => "TransactionSigningHashECDSA",
}

macro_rules! sha256_hasher {
    ($(struct $name:ident => $domain_sep:literal),+ $(,)? ) => {$(
        #[derive(Clone)]
        pub struct $name(sha2::Sha256);

        impl $name {
            #[inline]
            pub fn new() -> Self {
                use sha2::{Sha256, Digest};
                // We use Lazy in order to avoid rehashing it
                // in the future we can replace this with the correct initial state.
                static HASHER: Lazy<$name> = Lazy::new(|| {
                    // SHA256 doesn't natively support domain separation, so we hash it to make it constant size.
                    let mut tmp_state = Sha256::new();
                    tmp_state.update($domain_sep);
                    let mut out = Self(Sha256::new());
                    out.write(tmp_state.finalize());

                    out
                });
                (*HASHER).clone()
            }

            pub fn write<A: AsRef<[u8]>>(&mut self, data: A) {
                sha2::Digest::update(&mut self.0, data.as_ref());
            }

            #[inline(always)]
            pub fn finalize(self) -> crate::Blake2Hash {
                let mut out = [0u8; 32];
                out.copy_from_slice(sha2::Digest::finalize(self.0).as_slice());
                crate::Blake2Hash(out)
            }
        }
    impl_hasher!{ struct $name }
    )*};
}

macro_rules! blake2b_hasher {
    ($(struct $name:ident => $domain_sep:literal),+ $(,)? ) => {$(
        #[derive(Clone)]
        pub struct $name(blake2b_simd::State);

        impl $name {
            #[inline(always)]
            pub fn new() -> Self {
                Self(
                    blake2b_simd::Params::new()
                        .hash_length(32)
                        .key($domain_sep)
                        .to_state(),
                )
            }

            pub fn write<A: AsRef<[u8]>>(&mut self, data: A) {
                self.0.update(data.as_ref());
            }

            #[inline(always)]
            pub fn finalize(self) -> crate::Blake2Hash {
                let mut out = [0u8; 32];
                out.copy_from_slice(self.0.finalize().as_bytes());
                crate::Blake2Hash(out)
            }
        }
    impl_hasher!{ struct $name }
    )*};
}
macro_rules! impl_hasher {
    (struct $name:ident) => {
        impl HasherBase for $name {
            #[inline(always)]
            fn update<A: AsRef<[u8]>>(&mut self, data: A) -> &mut Self {
                self.write(data);
                self
            }
        }
        impl Hasher for $name {
            #[inline(always)]
            fn finalize(self) -> crate::Blake2Hash {
                // Call the method
                $name::finalize(self)
            }
            #[inline(always)]
            fn reset(&mut self) {
                *self = Self::new();
            }
        }
        impl Default for $name {
            #[inline(always)]
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

use {blake2b_hasher, impl_hasher, sha256_hasher};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectors() {
        let input_data = [
            &[],
            &[1][..],
            &[
                5, 199, 126, 44, 71, 32, 82, 139, 122, 217, 43, 48, 52, 112, 40, 209, 180, 83, 139, 231, 72, 48, 136, 48, 168, 226,
                133, 7, 60, 4, 160, 205,
            ][..],
            &[42; 64],
            &[0; 8][..],
        ];

        fn run_test_vector<H: Hasher>(input_data: &[&[u8]], hasher_new: impl FnOnce() -> H, expected: &[&str]) {
            let mut hasher = hasher_new();
            // We do not reset the hasher each time on purpose, this also tests incremental hashing.
            for (data, expected) in input_data.iter().zip(expected) {
                println!("data: {data:?}");
                let hash = hasher.update(data).clone().finalize();
                assert_eq!(hash.to_string(), *expected, "Type: {}", std::any::type_name::<H>());
            }
        }

        run_test_vector(
            &input_data,
            TransactionHash::new,
            &[
                "50272a9e37c728026f93d0eda6ab4467f627338b879076483c88d291193cb3bf",
                "f9bf7e04c712621a0f4bb75d763f9ef5f73af6c438fd15b80744393bc96398ad",
                "8e791f3edcc92b71b8de2778efbc4666ee5bd146acbe8723a55bca26b022b0e0",
                "a6dab1a3088548c62d13a082fa28e870fdbbe51adcd8c364e2ea37e473c04d81",
                "3b79b78b967233843ad30f707b165eb3d6a91af8338076be8755c46a963c3d1d",
            ],
        );
        run_test_vector(
            &input_data,
            TransactionID::new,
            &[
                "e5f65efda0894d2b0590c2e9e46e9acc03032f505a1522f5e8c78c5ec70b1d9c",
                "aea52cf5e5a13da13a52dd69abd636eb1b0f86e58bc1dda6b17886b94593415a",
                "a50a2f87bdce075740189e9e23907ae22b5addbd875ccb70c116811b1fa5fb18",
                "0db7a485f7013a346a8f7f5caf73d52ca3c3b5ee101ad8753adedd4235b7236b",
                "2afc9c855854b0a6e94a722c3451d0cdfc8c11748b78ef65b9786f87b48d0d07",
            ],
        );

        run_test_vector(
            &input_data,
            TransactionSigningHash::new,
            &[
                "34c75037ad62740d4b3228f88f844f7901c07bfacd55a045be518eabc15e52ce",
                "8523b0471bcbea04575ccaa635eef9f9114f2890bda54367e5ff8caa3878bf82",
                "a51c49d9eb3d13f9de16e1aa8d1ff17668d55633ce00f36a643ac714b0fb137f",
                "487f199ef74c3e893e85bd37770e6334575a2d4d113b2e10474593c49807de93",
                "6392adc33a8e24e9a0a0c4c5f07f9c1cc958ad40c16d7a9a276e374cebb4e32b",
            ],
        );
        run_test_vector(
            &input_data,
            TransactionSigningHashECDSA::new,
            &[
                "b31ad1fbbe41b0e2a90e07c84708b38ba581f0c0e9185416913a04fb6d342027",
                "c43e1f75ea9df6379b56a95074c2b6289ed8c5a01fff2d49d9d44ad5575c164b",
                "49085f99fa0084b5436663f757a5916b1e4290c3321707fb76921ed4e47844ec",
                "3f887e866428de813c1d0463b14eef3ca1363c8187e917dda1eee0ec5996490b",
                "56de89a8c75f0fee2de61b11ab05d0d42e29ed50879467cf128dd80800a52ada",
            ],
        );

        run_test_vector(
            &input_data,
            BlockHash::new,
            &[
                "a80b6aa20f20b15ebabe2b1949527f78a257594a732e774de637d85e6973a768",
                "5643023add641f9421187b8c9aa3c6c73227d5ec34131c61a08d35b43e7e4b65",
                "4dc3bf72045431e46f8839a7d390898f27c887fddd8637149bfb70f732f04334",
                "15d7648e69023dca65c949a61ea166192049f449c604523494813873b19918a7",
                "3ac41af8385ea5d902ce6d47f509b7accc9c631f1d57a719d777874467f6d877",
            ],
        );

        run_test_vector(
            &input_data,
            MerkleBranchHash::new,
            &[
                "4de3617db456d01248173f17ec58196e92fbd994b636476db4b875ed2ec84054",
                "5737cd8b6fca5a30c19a491323a14e6b7021641cb3f8875f10c7a2eafd3cf43f",
                "a49eeda61cc75e0a8e5915829752fe0ad97620d6d32de7c9883595b0810ca33e",
                "28f33681dcff1313674e07dacc2d74c3089f6d8cea7a4f8792a71fd870988ee5",
                "2d53a43a42020a5091c125230bcd8a4cf0eeb188333e68325d4bce58a1c75ca3",
            ],
        );
    }
}
