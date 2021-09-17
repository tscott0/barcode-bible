use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use barcoders::generators::image::Image;
use genpdf::{elements, fonts, style, Alignment, Element};

use crate::code::{Code, CodeType};

mod code;

const PDF_PATH: &'static str = "output.pdf";
const PDF_TITLE: &'static str = "Barcode Bible";
const PDF_MARGIN_PX: i32 = 10;

fn main() {
    let codes = vec![
        Code {
            code_type: CodeType::EAN8,
            barcode: "00045933".to_string(),
        },
        Code {
            code_type: CodeType::EAN13,
            barcode: "2000926398005".to_string(),
        },
    ];

    let processed: Vec<(&Code, String)> = codes
        .iter()
        .map(|c| {
            // let png = Image::png(80); // You must specify the height in pixels.
            let jpg = Image::jpeg(80); // You must specify the height in pixels.
            let image_dir = "images";
            let path = format!("{}/{}.jpeg", image_dir, c.barcode);

            let encoded = c
                .encode()
                .expect(format!("failed to encode image {}", path).as_str());

            let bytes = jpg
                .generate(&encoded[..])
                .expect(format!("failed to generate image {}", path).as_str());

            fs::create_dir_all(image_dir).expect(
                format!("failed to create directory {}", image_dir).as_str(),
            );

            let file = File::create(&Path::new(path.as_str()))
                .expect(format!("failed to create file {}", path).as_str());

            BufWriter::new(file)
                .write(&bytes[..])
                .expect(format!("failed to write file {}", path).as_str());

            (c, path)
        })
        .collect();

    let default_font = fonts::from_files("./fonts", "Go", None)
        .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);

    let bold_style =
        style::Style::from(doc.font_cache().default_font_family()).bold();

    // Change the default settings
    doc.set_title(PDF_TITLE);
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(PDF_MARGIN_PX);
    doc.set_page_decorator(decorator);

    doc.push(
        elements::Paragraph::new("Barcode Bible")
            .aligned(Alignment::Center)
            .styled(bold_style),
    );

    doc.push(elements::Break::new(1.5));

    processed.iter().for_each(|(c, p)| {
        doc.push(elements::Paragraph::default().string(c.barcode.clone()));
        doc.push(
            elements::Image::from_path(p)
                .expect("Unable to load image")
                .with_alignment(Alignment::Center),
        );
        doc.push(elements::Break::new(1.5));
    });

    // Render the document and write it to a file
    doc.render_to_file(PDF_PATH)
        .expect("Failed to write PDF file");
}
