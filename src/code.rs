use std::fmt;

use barcoders::error::Error;
use barcoders::sym::code128::Code128;
use barcoders::sym::ean13::EAN13;
use barcoders::sym::ean8::EAN8;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Code {
    EAN8(String),
    EAN13(String),
    CODE128(String),
}

impl Code {
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        return match self {
            Code::EAN8(b) => {
                EAN8::new(b[..7].to_string()).and_then(|b| Ok(b.encode()))
            }
            Code::EAN13(b) => {
                EAN13::new(b[..12].to_string()).and_then(|b| Ok(b.encode()))
            }
            Code::CODE128(b) => {
                let code128 = format!("\u{00C0}{}", b);
                Code128::new(code128).and_then(|b| Ok(b.encode()))
            }
        };
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Code::EAN8(b) => {
                write!(f, "EAN8_{}", b)
            }
            Code::EAN13(b) => {
                write!(f, "EAN13_{}", b)
            }
            Code::CODE128(b) => {
                write!(f, "CODE128_{}", b)
            }
        };
    }
}
