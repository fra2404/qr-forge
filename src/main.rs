use clap::Parser;
use image::{ImageBuffer, Rgb};
use qrcode::{QrCode, EcLevel};
use std::fs;
use svg::Document;
use svg::node::element::{Rectangle, Group};

mod gui_core;

#[derive(Parser)]
#[command(name = "qr-forge")]
#[command(about = "🔥 QR Forge - High-quality QR code generator with SVG support")]
#[command(version = "1.0.0")]
#[command(author = "Francesco")]
#[command(long_about = "QR Forge is a powerful QR code generator that supports scalable SVG formats and high-resolution bitmaps. Perfect for professional use, printing, and web.")]
struct Args {
    /// Website URL to generate QR code for
    #[arg(short, long)]
    url: Option<String>,

    /// Output file name (without extension)
    #[arg(short, long, default_value = "qrcode")]
    output: String,

    /// QR code size in pixels (width x height)
    #[arg(short, long, default_value = "800")]
    size: u32,

    /// Error correction level: L (low), M (medium), Q (quartile), H (high)
    #[arg(short, long, default_value = "H")]
    error_correction: String,

    /// Margin around QR code (in modules)
    #[arg(short, long, default_value = "4")]
    margin: u32,

    /// Output format: png, jpg, bmp, svg
    #[arg(short, long, default_value = "png")]
    format: String,

    /// QR code color (for SVG, hex format without #)
    #[arg(long, default_value = "000000")]
    color: String,

    /// Background color (for SVG, hex format without #)
    #[arg(long, default_value = "ffffff")]
    background_color: String,

    /// Launch GUI mode instead of CLI
    #[arg(long, action)]
    gui: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Check if GUI mode is requested
    if args.gui {
        println!("🚀 Launching QR Forge GUI...");
        return gui_core::run_gui().map_err(|e| e.into());
    }

    // CLI mode - URL is required
    let url_str = match &args.url {
        Some(url) => url,
        None => {
            eprintln!("❌ Error: URL is required in CLI mode. Use --url <URL> or --gui for GUI mode.");
            std::process::exit(1);
        }
    };

    // URL validation
    let url = validate_url(url_str)?;
    
    // Determine error correction level
    let ec_level = match args.error_correction.to_uppercase().as_str() {
        "L" => EcLevel::L,
        "M" => EcLevel::M,
        "Q" => EcLevel::Q,
        "H" => EcLevel::H,
        _ => return Err("Invalid error correction level. Use: L, M, Q, H".into()),
    };

    println!("🔧 Generating QR code for: {}", url);
    println!("📊 Parameters:");
    println!("   - Size: {}x{} pixels", args.size, args.size);
    println!("   - Error correction: {}", args.error_correction);
    println!("   - Margin: {} modules", args.margin);
    println!("   - Format: {}", args.format);

    // Generate QR code
    let qr_code = QrCode::with_error_correction_level(&url, ec_level)
        .map_err(|e| format!("Error generating QR code: {}", e))?;

    // Determine filename
    let filename = format!("{}.{}", args.output, args.format.to_lowercase());
    
    // Generate file based on format
    match args.format.to_lowercase().as_str() {
        "svg" => {
            generate_svg_qr(&qr_code, &filename, args.size, args.margin, &args.color, &args.background_color)?;
        }
        _ => {
            // Create high-resolution image for bitmap formats
            let image = generate_high_quality_image(&qr_code, args.size, args.margin)?;
            save_image(&image, &filename, &args.format)?;
        }
    }

    println!("✅ QR code generated successfully!");
    println!("📁 File saved as: {}", filename);
    
    // Show QR code statistics
    print_qr_stats(&qr_code, &url);

    Ok(())
}

fn validate_url(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else {
        format!("https://{}", input)
    };

    // Basic validation
    if !url.contains('.') {
        return Err("Invalid URL: must contain at least one dot".into());
    }

    Ok(url)
}

pub fn generate_high_quality_image(
    qr_code: &QrCode,
    size: u32,
    margin: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let qr_width = qr_code.width() as u32;
    let total_modules = qr_width + (margin * 2);
    let module_size = size / total_modules;
    let actual_size = module_size * total_modules;

    println!("📐 Technical details:");
    println!("   - QR modules: {}x{}", qr_width, qr_width);
    println!("   - Total modules (with margin): {}x{}", total_modules, total_modules);
    println!("   - Module size: {}x{} pixels", module_size, module_size);
    println!("   - Final size: {}x{} pixels", actual_size, actual_size);

    if module_size < 4 {
        eprintln!("⚠️  Warning: Very small module size ({}px). Consider increasing total size.", module_size);
    }

    let mut image = ImageBuffer::new(actual_size, actual_size);

    // Fill with white
    for pixel in image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Draw QR code
    for y in 0..qr_width {
        for x in 0..qr_width {
            if qr_code[(x as usize, y as usize)] == qrcode::Color::Dark {
                // Calculate position with margin
                let start_x = (x + margin) * module_size;
                let start_y = (y + margin) * module_size;

                // Draw a module (black square)
                for dy in 0..module_size {
                    for dx in 0..module_size {
                        let px = start_x + dx;
                        let py = start_y + dy;
                        
                        if px < actual_size && py < actual_size {
                            image.put_pixel(px, py, Rgb([0, 0, 0]));
                        }
                    }
                }
            }
        }
    }

    Ok(image)
}

fn save_image(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    filename: &str,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match format.to_lowercase().as_str() {
        "png" => {
            image.save_with_format(filename, image::ImageFormat::Png)?;
        }
        "jpg" | "jpeg" => {
            image.save_with_format(filename, image::ImageFormat::Jpeg)?;
        }
        "bmp" => {
            image.save_with_format(filename, image::ImageFormat::Bmp)?;
        }
        _ => {
            return Err(format!("Unsupported format: {}. Use png, jpg, bmp, or svg", format).into());
        }
    }

    // Show file information
    if let Ok(metadata) = fs::metadata(filename) {
        let size_kb = metadata.len() as f64 / 1024.0;
        println!("📊 File size: {:.2} KB", size_kb);
    }

    Ok(())
}

fn print_qr_stats(qr_code: &QrCode, url: &str) {
    println!("\n📈 QR Code Statistics:");
    println!("   - Version: {:?}", qr_code.version());
    println!("   - Matrix size: {}x{} modules", qr_code.width(), qr_code.width());
    println!("   - URL length: {} characters", url.len());
    println!("   - Error correction level: {:?}", qr_code.error_correction_level());
    
    // Maximum capacity for this version
    let max_capacity = get_max_capacity(qr_code.version(), qr_code.error_correction_level());
    println!("   - Maximum capacity: {} characters", max_capacity);
    println!("   - Capacity usage: {:.1}%", (url.len() as f64 / max_capacity as f64) * 100.0);
    
    println!("\n💡 Usage tips:");
    println!("   - Test the QR code with different readers");
    println!("   - Ensure it's readable even when printed");
    println!("   - For printing, use at least 2.5cm x 2.5cm");
    
    if url.len() > max_capacity * 80 / 100 {
        println!("   ⚠️  URL close to capacity limit - consider shortening it");
    }
}

fn get_max_capacity(version: qrcode::Version, ec_level: EcLevel) -> usize {
    // Approximate capacities for alphanumeric data based on QR version
    // We use matrix modules to determine capacity
    let modules = match version {
        qrcode::Version::Normal(v) => (4 * v + 17) as u8,
        qrcode::Version::Micro(_) => 21, // Simplification for micro QR
    };
    
    match (modules, ec_level) {
        (21, EcLevel::L) => 25,
        (21, EcLevel::M) => 20,
        (21, EcLevel::Q) => 16,
        (21, EcLevel::H) => 10,
        (25, EcLevel::L) => 47,
        (25, EcLevel::M) => 38,
        (25, EcLevel::Q) => 29,
        (25, EcLevel::H) => 20,
        (29, EcLevel::L) => 77,
        (29, EcLevel::M) => 61,
        (29, EcLevel::Q) => 47,
        (29, EcLevel::H) => 35,
        (33, EcLevel::L) => 114,
        (33, EcLevel::M) => 90,
        (33, EcLevel::Q) => 67,
        (33, EcLevel::H) => 50,
        _ => 1000, // Default value for higher versions
    }
}

pub fn generate_svg_qr(
    qr_code: &QrCode,
    filename: &str,
    size: u32,
    margin: u32,
    qr_color: &str,
    bg_color: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let qr_width = qr_code.width() as u32;
    let total_modules = qr_width + (margin * 2);
    let module_size = size / total_modules;
    let actual_size = module_size * total_modules;

    println!("📐 SVG technical details:");
    println!("   - QR modules: {}x{}", qr_width, qr_width);
    println!("   - Total modules (with margin): {}x{}", total_modules, total_modules);
    println!("   - Module size: {}x{} SVG units", module_size, module_size);
    println!("   - Final size: {}x{} SVG units", actual_size, actual_size);
    println!("   - QR color: #{}", qr_color);
    println!("   - Background color: #{}", bg_color);

    // Create SVG document
    let mut document = Document::new()
        .set("viewBox", (0, 0, actual_size, actual_size))
        .set("width", actual_size)
        .set("height", actual_size)
        .set("xmlns", "http://www.w3.org/2000/svg");

    // Colored background
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", actual_size)
        .set("height", actual_size)
        .set("fill", format!("#{}", bg_color));
    
    document = document.add(background);

    // Group for all QR code modules with custom color
    let mut qr_group = Group::new()
        .set("fill", format!("#{}", qr_color))
        .set("shape-rendering", "crispEdges"); // For sharp edges

    // Draw QR code modules
    for y in 0..qr_width {
        for x in 0..qr_width {
            if qr_code[(x as usize, y as usize)] == qrcode::Color::Dark {
                let rect_x = (x + margin) * module_size;
                let rect_y = (y + margin) * module_size;
                
                let module_rect = Rectangle::new()
                    .set("x", rect_x)
                    .set("y", rect_y)
                    .set("width", module_size)
                    .set("height", module_size);
                
                qr_group = qr_group.add(module_rect);
            }
        }
    }

    document = document.add(qr_group);

    // Save SVG file
    std::fs::write(filename, document.to_string())?;

    // Show file information
    if let Ok(metadata) = fs::metadata(filename) {
        let size_kb = metadata.len() as f64 / 1024.0;
        println!("📊 SVG file size: {:.2} KB", size_kb);
    }

    println!("✨ SVG QR code generated! Infinitely scalable without quality loss.");

    Ok(())
}
