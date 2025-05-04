use bitcode::{DecodeOwned, Encode};
use crate::{ContentType, Decodes, Encodes, Format, FormatType};
use bytes::Bytes;



/// Encode and Decode Bitcode with [`bitcode`].
pub struct BitcodeEncoding;

impl ContentType for BitcodeEncoding {
    const CONTENT_TYPE: &'static str = "application/bitcode";
}

impl FormatType for BitcodeEncoding {
    const FORMAT_TYPE: Format = Format::Binary;
}

impl<T> Encodes<T> for BitcodeEncoding
where
    T: Encode,
{
    type Error = bitcode::Error;

    fn encode(value: &T) -> Result<Bytes, Self::Error> {
        Ok(Bytes::from(bitcode::encode(value)))
    }
}

impl<T> Decodes<T> for BitcodeEncoding
where
    T: DecodeOwned,
{
    type Error = bitcode::Error;

    fn decode(bytes: Bytes) -> Result<T, Self::Error> {
        bitcode::decode(bytes.as_ref())
    }
}

/// Pass arguments and receive responses using `bitcode` in a `POST` request.
pub type Bitcode = crate::codec::post::Post<BitcodeEncoding>;

/// Pass arguments and receive responses using `bitcode` in the body of a `PATCH` request.
/// **Note**: Browser support for `PATCH` requests without JS/WASM may be poor.
/// Consider using a `POST` request if functionality without JS/WASM is required.
pub type PatchBitcode = crate::codec::patch::Patch<BitcodeEncoding>;

/// Pass arguments and receive responses using `bitcode` in the body of a `PUT` request.
/// **Note**: Browser support for `PUT` requests without JS/WASM may be poor.
/// Consider using a `POST` request if functionality without JS/WASM is required.
pub type PutBitcode = crate::codec::put::Put<BitcodeEncoding>;