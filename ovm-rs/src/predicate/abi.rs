extern crate ethabi;

use ethabi::Token;

pub trait Encodable {
    fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }

    fn to_tuple(&self) -> Vec<Token>;
}

pub trait Decodable {
    type Ok;
    fn from_tuple(tuple: &[Token]) -> Self::Ok;

    fn from_abi(data: &[u8]) -> Option<Self::Ok>;
}
