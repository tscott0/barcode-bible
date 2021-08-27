use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use barcoders::error::Error;
use barcoders::generators::image::Image;
use barcoders::sym::ean13::EAN13;
use barcoders::sym::ean8::EAN8;

pub enum CodeType {
    EAN8,
    EAN13,
}

pub struct Code {
    pub code_type: CodeType,
    pub barcode: String,
}

impl Code {
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        return match self.code_type {
            CodeType::EAN8 => {
                EAN8::new(&self.barcode[..7]).and_then(|b| Ok(b.encode()))
            }
            CodeType::EAN13 => {
                EAN13::new(&self.barcode[..12]).and_then(|b| Ok(b.encode()))
            }
        };
    }
}

