use crate::code::Code;

#[derive(Debug, Clone)]
pub struct Product {
    pub barcode: Code,
    pub name: String,
}

impl Product {
    pub fn new<S: Into<String>>(code: S, name: S) -> Product {
        let c = code.into();

        let ean = match c.len() {
            8 => Code::EAN8(c),
            13 => Code::EAN13(c),
            22 => Code::CODE128(c),
            l => panic!("Code {} has unexpected length: {}", c, l),
        };

        Product {
            barcode: ean,
            name: name.into(),
        }
    }
}
