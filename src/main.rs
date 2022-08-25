use crate::code::Code;
use barcoders::generators::image::{Color, Image, Rotation};
use chrono;
use clap::Parser;
use genpdf::{elements, fonts, style, Alignment, Document, Element};
use product::Product;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{env, fs};
use toml;

mod code;
mod product;

const DEFAULT_PDF_PATH: &'static str = "barcodes.pdf";
const DEFAULT_PDF_TITLE: &'static str = "Barcode Bible";
const PDF_MARGIN_PX: i32 = 10;

/// Generate a barcode PDF
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Config file to read (TOML)
    #[clap(short, long, value_parser)]
    config: String,

    /// output PDF path
    #[clap(short, long, value_parser, default_value=DEFAULT_PDF_PATH)]
    output_path: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    products: Vec<ConfigProduct>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigProduct {
    pub barcode: String,
    pub name: String,
}

const COLUMNS: usize = 2;

const JPEG_RENDER_HEIGHT: u32 = 4;
const JPEG_RENDER_BAR_WIDTH: u32 = 6;
const BARCODE_X_SCALE: f32 = 1.0;
const BARCODE_Y_SCALE: f32 = 40.0;

fn main() {
    let args = Args::parse();

    let cfg = read_config(args.config).expect("failed to read config file");

    // Create a temporary directory to store barcode images
    // should be removed when it goes out of scope
    let tmp_image_dir = env::temp_dir();

    let processed: Vec<(Product, String)> = cfg
        .products
        .iter()
        .map(|p| Product::new(p.barcode.clone(), p.name.clone()))
        .map(|p| {
            let jpg = Image::JPEG {
                height: JPEG_RENDER_HEIGHT,
                xdim: JPEG_RENDER_BAR_WIDTH,
                rotation: Rotation::Zero,
                foreground: Color::black(),
                background: Color::white(),
            };

            let encoded = p.barcode.encode().expect(
                format!("failed to encode barcode {}", p.barcode).as_str(),
            );

            let bytes = jpg.generate(&encoded[..]).expect(
                format!("failed to generate jpeg for barcode {}", p.barcode)
                    .as_str(),
            );

            let file_name = format!("{}.jpeg", p.barcode);
            let image_path =
                tmp_image_dir.as_path().with_file_name(file_name.clone());

            let file = File::create(image_path.clone()).expect(
                format!("failed to create file {}", file_name).as_str(),
            );

            BufWriter::new(file)
                .write(&bytes[..])
                .expect(format!("failed to write file {}", file_name).as_str());

            let image_path_string = image_path.to_str().unwrap().to_string();
            (p, image_path_string)
        })
        .collect();

    let mut doc = create_document();
    write_header(&mut doc);
    write_product_table(&mut doc, processed);

    println!("Writing PDF to {}", args.output_path);
    // Render the document and write it to a file
    doc.render_to_file(args.output_path)
        .expect("Failed to write PDF file");

    println!("Done!");
}

fn read_config(path: String) -> std::io::Result<Config> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

fn create_document() -> Document {
    let default_font = fonts::from_files("./fonts", "Go", None)
        .expect("Failed to load the default font family");

    let mut doc = Document::new(default_font);
    doc.set_title(DEFAULT_PDF_TITLE);
    let mut page_decorator = genpdf::SimplePageDecorator::new();
    page_decorator.set_margins(PDF_MARGIN_PX);
    doc.set_page_decorator(page_decorator);
    doc
}

fn write_header(doc: &mut Document) {
    let bold_style =
        style::Style::from(doc.font_cache().default_font_family()).bold();

    doc.push(
        elements::Paragraph::new("Barcodes")
            .aligned(Alignment::Center)
            .styled(bold_style),
    );
    doc.push(
        elements::Paragraph::new(
            chrono::offset::Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
        .aligned(Alignment::Center)
        .styled(bold_style),
    );

    doc.push(elements::Break::new(2));
}

fn write_product_table(doc: &mut Document, products: Vec<(Product, String)>) {
    let mut table = elements::TableLayout::new(vec![1, 1]);
    products.chunks(COLUMNS).for_each(|both| {
        let mut r = table.row();

        both.iter().for_each(|(product, path)| {
            let barcode_x_scale = if let Code::CODE128(_) = product.barcode {
                BARCODE_X_SCALE * 0.5
            } else {
                BARCODE_X_SCALE
            };

            let list_layout = elements::LinearLayout::vertical()
                .element(
                    elements::Paragraph::default().string(product.name.clone()),
                )
                .element(
                    elements::Paragraph::default()
                        .string(product.barcode.to_string()),
                )
                .element(elements::Break::new(1))
                .element(
                    elements::Image::from_path(path)
                        .expect("Unable to load image")
                        // .with_alignment(Alignment::Center)
                        .with_scale(genpdf::Scale::new(
                            barcode_x_scale,
                            BARCODE_Y_SCALE,
                        )),
                )
                .element(elements::Break::new(3));
            r.push_element(list_layout);
        });

        let padding_columns = if both.len() < COLUMNS {
            COLUMNS - both.len()
        } else {
            0
        };

        for _ in 0..padding_columns {
            r.push_element(elements::Break::new(1));
        }

        r.push().expect("Invalid table row");
    });

    doc.push(table);
}
