mod code;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use barcoders::error::Error;
use barcoders::generators::image::Image;
use barcoders::sym::ean13::EAN13;
use barcoders::sym::ean8::EAN8;
use crate::code::{CodeType, Code};
use genpdf::fonts::{Builtin, FontFamily, Font};
use genpdf::fonts;

fn main() {
    let codes = vec!(
        Code { code_type: CodeType::EAN8, barcode: "00045933".to_string() },
        Code { code_type: CodeType::EAN13, barcode: "2000926398005".to_string() },
    );

    codes.iter().for_each(|c| {
        let png = Image::png(80); // You must specify the height in pixels.
        let encoded = c.encode().unwrap();

        let bytes = png.generate(&encoded[..]).unwrap();

        let path = format!("images/{}.png", c.barcode);
        let file = File::create(&Path::new(path.as_str())).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&bytes[..]).unwrap();
    });


    const FONT_DIRS: &[&str] = &[
        "/System/Library/Fonts/",
    ];

    const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

    // Load a font from the file system
    // let font_family = genpdf::fonts::from_files("./", "Go", None)
    //     .expect("Failed to load font family");
    // let font_dir = FONT_DIRS
    //     .iter()
    //     .filter(|path| std::path::Path::new(path).exists())
    //     .next()
    //     .expect("Could not find font directory");
    let default_font =
        fonts::from_files("./fonts", "Go", None)
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    // Change the default settings
    doc.set_title("Demo document");
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    // Add one or more elements
    doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
    // Render the document and write it to a file
    doc.render_to_file("output.pdf").expect("Failed to write PDF file");
}
