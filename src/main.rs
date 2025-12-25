use clap::Parser;
use pdfium_render::prelude::*;
use std::path::PathBuf;
use std::fmt;

#[derive(Parser, Debug)]
#[command(name = "pdf2other")]
#[command(about = "将PDF文件的指定页面范围转换为图片", long_about = None)]
struct Args {
    /// PDF文件路径
    #[arg(value_name = "PDF_PATH")]
    pdf_path: PathBuf,

    /// 页码范围，支持格式如 1-5 或 1..5
    #[arg(short, long, value_name = "RANGE")]
    pages: String,

    /// 输出格式：png 或 jpeg
    #[arg(short, long, default_value = "png")]
    format: String,

    /// 渲染DPI
    #[arg(long, default_value_t = 150)]
    dpi: u32,
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
            PageRangeError::InvalidFormat => write!(f, "页码范围格式无效，请使用 1-5 或 1..5 格式"),
            PageRangeError::InvalidPageNumber => write!(f, "页码无效，页码必须大于0"),
            PageRangeError::EmptyRange => write!(f, "页码范围无效，起始页码不能大于结束页码"),
        }
    }
}

impl std::error::Error for PageRangeError {}

/// 解析页码范围字符串，返回页面索引列表（1-based）
fn parse_page_range(range_str: &str) -> Result<Vec<u32>, PageRangeError> {
    let range_str = range_str.trim();
    
    // 支持 1-5 或 1..5 格式
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

    // 验证PDF文件存在
    if !args.pdf_path.exists() {
        return Err(format!("PDF文件不存在: {}", args.pdf_path.display()).into());
    }

    // 验证输出格式
    let format = args.format.to_lowercase();
    if format != "png" && format != "jpeg" && format != "jpg" {
        return Err(format!("不支持的输出格式: {}，仅支持 png 或 jpeg", format).into());
    }

    // 解析页码范围
    let page_numbers = parse_page_range(&args.pages)?;

    println!("正在打开PDF文件: {}", args.pdf_path.display());
    
    // 初始化PDFium库（使用静态链接）
    // let pdfium_bindings = Pdfium::bind_to_statically_linked_library()
    //     .map_err(|e| {
    //         format!(
    //             "无法初始化PDFium静态库: {}\n\n\
    //             这通常是因为找不到PDFium静态库文件。\n\
    //             请确保：\n\
    //             1. 已设置 PDFIUM_STATIC_LIB_PATH 环境变量指向包含静态库的目录\n\
    //             2. 或者将静态库文件（pdfium.lib）放在项目根目录\n\
    //             3. Windows上需要 .lib 文件，Linux/macOS上需要 .a 文件\n\
    //             4. 可以从 https://github.com/bblanchon/pdfium-binaries 下载预编译版本\n\
    //             5. 参考 https://crates.io/crates/pdfium-render 了解详细说明",
    //             e
    //         )
    //     })?;
    
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library().unwrap());
    
    // 打开PDF文件
    let pdf = pdfium.load_pdf_from_file(&args.pdf_path, None)
        .map_err(|e| format!("无法打开PDF文件: {}", e))?;

    let total_pages = pdf.pages().len();
    println!("PDF总页数: {}", total_pages);

    // 验证页码范围
    if let Some(&max_page) = page_numbers.iter().max() {
        if max_page > total_pages as u32 {
            return Err(format!(
                "页码范围超出PDF总页数: 最大页码 {}，PDF总页数 {}",
                max_page, total_pages
            ).into());
        }
    }

    // 获取输出目录和文件名
    let output_dir = args.pdf_path.parent()
        .ok_or("无法获取PDF文件所在目录")?;
    let pdf_stem = args.pdf_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("无法获取PDF文件名")?;

    // 确定文件扩展名
    let ext = if format == "jpg" { "jpeg" } else { &format };

    println!("开始转换，共 {} 页", page_numbers.len());

    // 转换每一页
    for &page_num in &page_numbers {
        // PDF页面索引从0开始，需要转换为u16
        let page_index = (page_num - 1) as u16;
        
        println!("正在转换第 {} 页...", page_num);
        
        let page = pdf.pages().get(page_index)
            .map_err(|e| format!("无法获取第 {} 页: {}", page_num, e))?;

        // 渲染页面为图片
        // 计算渲染尺寸（基于DPI）
        let width_points = page.width().value;
        let height_points = page.height().value;
        let scale = args.dpi as f32 / 72.0; // 72 DPI是PDF的标准DPI
        let width_pixels = (width_points * scale) as i32;
        let height_pixels = (height_points * scale) as i32;

        let bitmap = page.render_with_config(
            &PdfRenderConfig::new()
                .set_target_width(width_pixels)
                .set_maximum_height(height_pixels)
        ).map_err(|e| format!("无法渲染第 {} 页: {}", page_num, e))?;

        // 获取像素数据
        // pdfium-render返回的bitmap通常是BGRA格式（每像素4字节）
        let raw_pixels = bitmap.as_raw_bytes();
        let width_u32 = width_pixels as u32;
        let height_u32 = height_pixels as u32;
        
        // 检查像素格式：如果是BGRA（每像素4字节），需要转换为RGB（每像素3字节）
        let rgb_pixels = if raw_pixels.len() == (width_u32 * height_u32 * 4) as usize {
            // BGRA格式，转换为RGB
            raw_pixels
                .chunks_exact(4)
                .flat_map(|bgra| {
                    // BGRA -> RGB: 跳过Alpha通道，交换B和R
                    [bgra[2], bgra[1], bgra[0]] // BGR -> RGB
                })
                .collect::<Vec<u8>>()
        } else if raw_pixels.len() == (width_u32 * height_u32 * 3) as usize {
            // 已经是RGB格式
            raw_pixels.to_vec()
        } else {
            return Err(format!(
                "不支持的像素格式（第 {} 页）: 期望 {} 或 {} 字节，实际 {} 字节",
                page_num,
                width_u32 * height_u32 * 3,
                width_u32 * height_u32 * 4,
                raw_pixels.len()
            ).into());
        };

        // 创建image::RgbImage
        let img = image::RgbImage::from_raw(
            width_u32,
            height_u32,
            rgb_pixels,
        ).ok_or(format!("无法创建图片（第 {} 页）", page_num))?;

        // 生成输出文件名
        let output_filename = format!("{}_page_{}.{}", pdf_stem, page_num, ext);
        let output_path = output_dir.join(&output_filename);

        // 保存图片
        match ext {
            "png" => {
                img.save(&output_path)
                    .map_err(|e| format!("无法保存PNG文件 {}: {}", output_path.display(), e))?;
            }
            "jpeg" => {
                let rgb_img = image::DynamicImage::ImageRgb8(img);
                rgb_img.save(&output_path)
                    .map_err(|e| format!("无法保存JPEG文件 {}: {}", output_path.display(), e))?;
            }
            _ => {
                return Err(format!("不支持的格式: {}", ext).into());
            }
        }

        println!("已保存: {}", output_path.display());
    }

    println!("转换完成！共转换 {} 页", page_numbers.len());
    Ok(())
}
