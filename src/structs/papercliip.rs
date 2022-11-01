use ethers::{
    abi::{
        decode, encode, ethereum_types::FromDecStrErr, AbiArrayType, AbiDecode, AbiEncode, AbiType,
        Detokenize, InvalidOutputType, ParamType, Token, Tokenizable,
    },
    prelude::*,
    utils::hex::FromHexError,
};
use paperclip::{
    actix::OperationModifier,
    v2::{
        models::{DataType, DefaultSchemaRaw},
        schema::TypedData,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    str::FromStr,
};

use super::TypedSession;

macro_rules! impl_type_simple {
    ($ty:ty) => {
        impl TypedData for $ty {}
    };
    ($ty:ty, $dt:expr) => {
        impl TypedData for $ty {
            fn data_type() -> DataType {
                $dt
            }
        }
    };
    ($ty:ty, $dt:expr, $df:expr) => {
        impl TypedData for $ty {
            fn data_type() -> DataType {
                $dt
            }
            fn format() -> Option<DataTypeFormat> {
                Some($df)
            }
        }
    };
}

macro_rules! impl_abi_codec {
    ($($name:ty),*) => {
        $(
            impl AbiEncode for $name {
                fn encode(self) -> Vec<u8> {
                    let token = self.into_token();
                    encode(&[token]).into()
                }
            }
            impl AbiDecode for $name {
                fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, AbiError> {
                    let tokens = decode(
                        &[Self::param_type()], bytes.as_ref()
                    )?;
                    Ok(<Self as Detokenize>::from_tokens(tokens)?)
                }
            }
        )*
    };
}

macro_rules! impl_abi_type {
    ($($name:ty => $var:ident $(($value:expr))? ),*) => {
        $(
            impl AbiType for $name {
                fn param_type() -> ParamType {
                    ParamType::$var $( ($value) )?
                }
            }

            impl AbiArrayType for $name {}
        )*
    };
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedAddress(pub Address);
impl_type_simple!(TypedAddress, DataType::String);

impl From<Address> for TypedAddress {
    fn from(value: Address) -> TypedAddress {
        TypedAddress(value)
    }
}

impl Tokenizable for TypedAddress {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        match token {
            Token::Address(data) => Ok(TypedAddress(data)),
            other => Err(InvalidOutputType(format!(
                "Expected `Address`, got {:?}",
                other
            ))),
        }
    }

    fn into_token(self) -> Token {
        Token::Address(self.0)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedH256(pub H256);
impl_type_simple!(TypedH256, DataType::String);

impl From<H256> for TypedH256 {
    fn from(value: H256) -> TypedH256 {
        TypedH256(value)
    }
}

impl Tokenizable for TypedH256 {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        Ok(TypedH256::from(H256::from_token(token).unwrap()))
    }

    fn into_token(self) -> Token {
        self.0.into_token()
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedBytes(pub Bytes);
impl_type_simple!(TypedBytes, DataType::String);

impl From<Bytes> for TypedBytes {
    fn from(value: Bytes) -> TypedBytes {
        TypedBytes(value)
    }
}

impl Tokenizable for TypedBytes {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        match token {
            Token::Bytes(s) => Ok(s.into()),
            other => Err(InvalidOutputType(format!(
                "Expected `Bytes`, got {:?}",
                other
            ))),
        }
    }

    fn into_token(self) -> Token {
        Token::Bytes(self.0.to_vec())
    }
}

impl From<Vec<u8>> for TypedBytes {
    fn from(src: Vec<u8>) -> Self {
        Self(src.into())
    }
}

impl FromStr for TypedBytes {
    type Err = ParseBytesError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(TypedBytes(Bytes::from_str(value).unwrap()))
    }
}

macro_rules! eth_wrapped_uint_tokenizable {
    ($uint: ident, $name: expr) => {
        impl Tokenizable for $uint {
            fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
                match token {
                    Token::Int(data) | Token::Uint(data) => {
                        Ok($uint(::std::convert::TryInto::try_into(data).unwrap()))
                    }
                    other => Err(InvalidOutputType(format!(
                        "Expected `{}`, got {:?}",
                        $name, other
                    ))
                    .into()),
                }
            }

            fn into_token(self) -> Token {
                Token::Uint(self.0.into())
            }
        }
    };
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedU256(pub U256);
impl_type_simple!(TypedU256, DataType::String);

impl From<U256> for TypedU256 {
    fn from(value: U256) -> TypedU256 {
        TypedU256(value)
    }
}

impl TypedU256 {
    pub fn from_dec_str(value: &str) -> Result<Self, FromDecStrErr> {
        Ok(TypedU256(U256::from_dec_str(value).unwrap()))
    }
}

impl From<i32> for TypedU256 {
    fn from(value: i32) -> TypedU256 {
        TypedU256(U256::from(value))
    }
}

impl From<i64> for TypedU256 {
    fn from(value: i64) -> TypedU256 {
        TypedU256(U256::from(value))
    }
}

impl From<TypedU256> for [u8; 32] {
    fn from(number: TypedU256) -> Self {
        let mut arr = [0u8; 32];
        number.0.to_big_endian(&mut arr);
        arr
    }
}

impl From<[u8; 32]> for TypedU256 {
    fn from(bytes: [u8; 32]) -> Self {
        TypedU256(U256::from(bytes))
    }
}

impl FromStr for TypedU256 {
    type Err = FromHexError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(TypedU256(U256::from_str(value).unwrap()))
    }
}

impl Display for TypedU256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl_abi_codec!(TypedAddress, TypedH256, TypedBytes, TypedU256);
impl_abi_type!(
    TypedAddress => Address,
    TypedH256 => FixedBytes(32),
    TypedBytes => Bytes,
    TypedU256 => Uint(256)
);
eth_wrapped_uint_tokenizable!(TypedU256, "U256");

impl_type_simple!(TypedSession);
impl OperationModifier for TypedSession {
    fn update_definitions(_map: &mut BTreeMap<String, DefaultSchemaRaw>) {}
}
