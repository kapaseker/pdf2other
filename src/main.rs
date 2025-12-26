use clap::Parser;
use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::fmt;

#[derive(Parser, Debug)]
#[command(name = "pdf2other")]
#[command(about = "Convert a specified page range of a PDF into images", long_about = None)]
struct Args {
    /// Path to the PDF file
    #[arg(value_name = "PDF_PATH")]
    pdf_path: PathBuf,

    /// Page range, supports formats like 1-5 or 1..5
    #[arg(short, long, value_name = "RANGE")]
    pages: String,

    /// Output format: png or jpeg
    #[arg(short, long, default_value = "png")]
    format: String,

    /// Render DPI
    #[arg(long, default_value_t = 480)]
    dpi: u32,

    /// Output directory; defaults to a directory with the same name as the PDF file
    #[arg(short, long, value_name = "DIR")]
    dir: Option<PathBuf>,
}

#[derive(Debug)]
enum PageRangeError {
    InvalidFormat,
    InvalidPageNumber,
    EmptyRange,
}

impl fmt::Display for PageRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PageRangeError::InvalidFormat => write!(f, "Invalid page range format. Use 1-5 or 1..5."),
            PageRangeError::InvalidPageNumber => write!(f, "Invalid page number. Page numbers must be greater than 0."),
            PageRangeError::EmptyRange => write!(f, "Invalid page range. The start page must not be greater than the end page."),
        }
    }
}

impl std::error::Error for PageRangeError {}

/// Parse a page range string and return a list of page numbers (1-based).
fn parse_page_range(range_str: &str) -> Result<Vec<u32>, PageRangeError> {
    let range_str = range_str.trim();
    
    // Supports 1-5 or 1..5 formats
    let separator = if range_str.contains("..") {
        ".."
    } else if range_str.contains("-") {
        "-"
    } else {
        return Err(PageRangeError::InvalidFormat);
    };

    let parts: Vec<&str> = range_str.split(separator).collect();
    if parts.len() != 2 {
        return Err(PageRangeError::InvalidFormat);
    }

    let start: u32 = parts[0]
        .trim()
        .parse()
        .map_err(|_| PageRangeError::InvalidPageNumber)?;
    let end: u32 = parts[1]
        .trim()
        .parse()
        .map_err(|_| PageRangeError::InvalidPageNumber)?;

    if start == 0 || end == 0 {
        return Err(PageRangeError::InvalidPageNumber);
    }

    if start > end {
        return Err(PageRangeError::EmptyRange);
    }

    Ok((start..=end).collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validate that the PDF file exists
    if !args.pdf_path.exists() {
        return Err(format!("PDF file does not exist: {}", args.pdf_path.display()).into());
    }

    // Validate output format
    let format = args.format.to_lowercase();
    if format != "png" && format != "jpeg" && format != "jpg" {
        return Err(format!(
            "Unsupported output format: {}. Only png or jpeg are supported.",
            format
        )
        .into());
    }

    // Parse page range
    let page_numbers = parse_page_range(&args.pages)?;

    println!("Opening PDF file: {}", args.pdf_path.display());

    let pdfium = Pdfium::new(Pdfium::bind_to_system_library().unwrap());
    
    // Open the PDF file
    let pdf = pdfium.load_pdf_from_file(&args.pdf_path, None)
        .map_err(|e| format!("Failed to open PDF file: {}", e))?;

    let total_pages = pdf.pages().len();
    println!("Total pages in PDF: {}", total_pages);

    // Validate page range
    if let Some(&max_page) = page_numbers.iter().max() {
        if max_page > total_pages as u32 {
            return Err(format!(
                "Page range exceeds total pages in PDF: max page {}, total pages {}",
                max_page, total_pages
            ).into());
        }
    }

    // Get output directory and file name stem
    let pdf_stem = args.pdf_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Failed to get PDF file name")?;
    
    // Determine output directory
    let output_dir = if let Some(dir) = &args.dir {
        // If a directory is specified, use it
        dir.clone()
    } else {
        // Default: use a directory with the same name under the PDF's parent directory
        let pdf_parent = args.pdf_path.parent()
            .ok_or("Failed to get PDF parent directory")?;
        pdf_parent.join(pdf_stem)
    };
    
    // Create output directory (if it doesn't exist)
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory {}: {}", output_dir.display(), e))?;
    
    println!("Output directory: {}", output_dir.display());

    // Determine file extension
    let ext = if format == "jpg" { "jpeg" } else { &format };

    println!("Starting conversion: {} page(s)", page_numbers.len());

    // Convert each page
    for &page_num in &page_numbers {
        // PDF page indices are 0-based; convert to u16
        let page_index = (page_num - 1) as u16;
        
        println!("Converting page {}...", page_num);
        
        let page = pdf.pages().get(page_index)
            .map_err(|e| format!("Failed to get page {}: {}", page_num, e))?;

        // Render the page to an image
        // Compute render size (based on DPI)
        let width_points = page.width().value;
        let height_points = page.height().value;
        let scale = args.dpi as f32 / 72.0; // 72 DPI is the PDF standard DPI
        let width_pixels = (width_points * scale) as i32;
        let height_pixels = (height_points * scale) as i32;

        let bitmap = page.render_with_config(
            &PdfRenderConfig::new()
                .set_target_width(width_pixels)
                .set_maximum_height(height_pixels)
        ).map_err(|e| format!("Failed to render page {}: {}", page_num, e))?;

        // Get pixel data
        // The bitmap returned by pdfium-render is typically BGRA (4 bytes per pixel)
        let raw_pixels = bitmap.as_raw_bytes();
        let width_u32 = width_pixels as u32;
        let height_u32 = height_pixels as u32;
        
        // Check pixel format: if BGRA (4 bytes/pixel), convert to RGB (3 bytes/pixel)
        let rgb_pixels = if raw_pixels.len() == (width_u32 * height_u32 * 4) as usize {
            // BGRA format; convert to RGB
            raw_pixels
                .chunks_exact(4)
                .flat_map(|bgra| {
                    // BGRA -> RGB: skip alpha channel and swap B/R
                    [bgra[2], bgra[1], bgra[0]] // BGR -> RGB
                })
                .collect::<Vec<u8>>()
        } else if raw_pixels.len() == (width_u32 * height_u32 * 3) as usize {
            // Already in RGB format
            raw_pixels.to_vec()
        } else {
            return Err(format!(
                "Unsupported pixel format (page {}): expected {} or {} bytes, got {} bytes",
                page_num,
                width_u32 * height_u32 * 3,
                width_u32 * height_u32 * 4,
                raw_pixels.len()
            ).into());
        };

        // Create image::RgbImage
        let img = image::RgbImage::from_raw(
            width_u32,
            height_u32,
            rgb_pixels,
        ).ok_or(format!("Failed to create image (page {})", page_num))?;

        // Generate output file name
        let output_filename = format!("{}_page_{}.{}", pdf_stem, page_num, ext);
        let output_path = output_dir.join(&output_filename);

        // Save image
        match ext {
            "png" => {
                img.save(&output_path)
                    .map_err(|e| format!("Failed to save PNG file {}: {}", output_path.display(), e))?;
            }
            "jpeg" => {
                let rgb_img = image::DynamicImage::ImageRgb8(img);
                rgb_img.save(&output_path)
                    .map_err(|e| format!("Failed to save JPEG file {}: {}", output_path.display(), e))?;
            }
            _ => {
                return Err(format!("Unsupported format: {}", ext).into());
            }
        }

        println!("Saved: {}", output_path.display());
    }

    println!("Done! Converted {} page(s).", page_numbers.len());
    Ok(())
}
