use bincode::{de::Deserializer, Serializer};
use crate::{ContentType, Decodes, Encodes, Format, FormatType};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Encode and Decode Bincode with [`bincode`].
pub struct BincodeEncoding;

impl ContentType for BincodeEncoding {
    const CONTENT_TYPE: &'static str = "application/bincode";
}

impl FormatType for BincodeEncoding {
    const FORMAT_TYPE: Format = Format::Binary;
}

impl<T> Encodes<T> for BincodeEncoding
where
    T: Serialize,
{
    type Error = bincode::Error;

    fn encode(value: &T) -> Result<Bytes, Self::Error> {
        Ok(Bytes::from(bincode::serialize(value)?))
    }
}

impl<T> Decodes<T> for BincodeEncoding
where
    T: DeserializeOwned,
{
    type Error = bincode::Error;

    fn decode(bytes: Bytes) -> Result<T, Self::Error> {
        bincode::deserialize(bytes.as_ref())
    }
}

/// Pass arguments and receive responses using `bitcode` in a `POST` request.
pub type Bincode = crate::codec::post::Post<BincodeEncoding>;

/// Pass arguments and receive responses using `bitcode` in the body of a `PATCH` request.
/// **Note**: Browser support for `PATCH` requests without JS/WASM may be poor.
/// Consider using a `POST` request if functionality without JS/WASM is required.
pub type PatchBincode = crate::codec::patch::Patch<BincodeEncoding>;

/// Pass arguments and receive responses using `bitcode` in the body of a `PUT` request.
/// **Note**: Browser support for `PUT` requests without JS/WASM may be poor.
/// Consider using a `POST` request if functionality without JS/WASM is required.
pub type PutBincode = crate::codec::put::Put<BincodeEncoding>;