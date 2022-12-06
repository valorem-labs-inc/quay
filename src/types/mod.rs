// Reference: https://github.com/ledgerwatch/interfaces/blob/master/src/lib.rs
use arrayref::array_ref;

// Setup From traits allowing the conversion between proto types and ethers types.
tonic::include_proto!("quay");

// Macro allowing for proto types to be converted into numbers (and vice versa), moving
// through the fixed hash type first.
macro_rules! into_from {
    ($proto:ty, $hash:ty, $num:ty) => {
        impl From<$num> for $proto {
            fn from(value: $num) -> Self {
                Self::from(<$hash>::from(<[u8; <$hash>::len_bytes()]>::from(value)))
            }
        }

        impl From<$proto> for $num {
            fn from(value: $proto) -> Self {
                Self::from(<$hash>::from(value).0)
            }
        }
    };
}

// TODO handle H40, H96 and H160
into_from!(H128, ethers::types::H128, ethers::types::U128);
into_from!(H256, ethers::types::H256, ethers::types::U256);

// Ethers will always upscale the types if required (i.e. it doesn't define a type small enough
// for it)
impl From<ethers::types::H64> for H40 {
    fn from(value: ethers::types::H64) -> Self {
        Self {
            hi: u32::from_be_bytes(*array_ref!(value, 0, 4)),
            lo: u32::from_be_bytes(*array_ref!(value, 4, 4)),
        }
    }
}

impl From<H40> for ethers::types::H64 {
    fn from(value: H40) -> Self {
        let mut v = [0; Self::len_bytes()];
        v[..4].copy_from_slice(&value.hi.to_be_bytes());
        v[4..].copy_from_slice(&value.lo.to_be_bytes());
        v.into()
    }
}

// Ethers will always upscale the types if required (i.e. it doesn't define a type small enough
// for it)
impl From<ethers::types::H128> for H96 {
    fn from(value: ethers::types::H128) -> Self {
        Self {
            hi: u64::from_be_bytes(*array_ref!(value, 0, 8)),
            lo: u32::from_be_bytes(*array_ref!(value, 8, 4)),
        }
    }
}

impl From<H96> for ethers::types::H128 {
    fn from(value: H96) -> Self {
        let mut v = [0; Self::len_bytes()];
        v[..8].copy_from_slice(&value.hi.to_be_bytes());
        v[8..].copy_from_slice(&value.lo.to_be_bytes());
        v.into()
    }
}

impl From<ethers::types::H128> for H128 {
    fn from(value: ethers::types::H128) -> Self {
        Self {
            hi: u64::from_be_bytes(*array_ref!(value, 0, 8)),
            lo: u64::from_be_bytes(*array_ref!(value, 8, 8)),
        }
    }
}

impl From<H128> for ethers::types::H128 {
    fn from(value: H128) -> Self {
        let mut v = [0; Self::len_bytes()];
        v[..8].copy_from_slice(&value.hi.to_be_bytes());
        v[8..].copy_from_slice(&value.lo.to_be_bytes());
        v.into()
    }
}

impl From<ethers::types::H160> for H160 {
    fn from(value: ethers::types::H160) -> Self {
        Self {
            hi: Some(ethers::types::H128::from_slice(&value[..16]).into()),
            lo: u32::from_be_bytes(*array_ref!(value, 16, 4)),
        }
    }
}

impl From<H160> for ethers::types::H160 {
    fn from(value: H160) -> Self {
        type H = ethers::types::H128;

        let mut v = [0; Self::len_bytes()];
        v[..H::len_bytes()].copy_from_slice(H::from(value.hi.unwrap_or_default()).as_fixed_bytes());
        v[H::len_bytes()..].copy_from_slice(&value.lo.to_be_bytes());

        v.into()
    }
}

impl From<ethers::types::H256> for H256 {
    fn from(value: ethers::types::H256) -> Self {
        Self {
            hi: Some(ethers::types::H128::from_slice(&value[..16]).into()),
            lo: Some(ethers::types::H128::from_slice(&value[16..]).into()),
        }
    }
}

impl From<H256> for ethers::types::H256 {
    fn from(value: H256) -> Self {
        type H = ethers::types::H128;

        let mut v = [0; Self::len_bytes()];
        v[..H::len_bytes()].copy_from_slice(H::from(value.hi.unwrap_or_default()).as_fixed_bytes());
        v[H::len_bytes()..].copy_from_slice(H::from(value.lo.unwrap_or_default()).as_fixed_bytes());

        v.into()
    }
}
