use eframe::egui;
use image::{ImageBuffer, Rgb};
use qrcode::QrCode;

mod gui_core;
use gui_core::QrForgeGui;

pub fn generate_high_quality_image(
    qr_code: &QrCode,
    size: u32,
    margin: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let qr_width = qr_code.width() as u32;
    let total_modules = qr_width + (margin * 2);
    let module_size = size / total_modules;
    let actual_size = module_size * total_modules;

    let mut image = ImageBuffer::new(actual_size, actual_size);

    // Fill with white
    for pixel in image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Draw QR code
    for y in 0..qr_width {
        for x in 0..qr_width {
            if qr_code[(x as usize, y as usize)] == qrcode::Color::Dark {
                let start_x = (x + margin) * module_size;
                let start_y = (y + margin) * module_size;

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

pub fn generate_svg_qr(
    qr_code: &QrCode,
    filename: &str,
    size: u32,
    margin: u32,
    qr_color: &str,
    bg_color: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use svg::Document;
    use svg::node::element::{Rectangle, Group};
    
    let qr_width = qr_code.width() as u32;
    let total_modules = qr_width + (margin * 2);
    let module_size = size / total_modules;
    let actual_size = module_size * total_modules;

    let mut document = Document::new()
        .set("viewBox", (0, 0, actual_size, actual_size))
        .set("width", actual_size)
        .set("height", actual_size)
        .set("xmlns", "http://www.w3.org/2000/svg");

    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", actual_size)
        .set("height", actual_size)
        .set("fill", format!("#{}", bg_color));
    
    document = document.add(background);

    let mut qr_group = Group::new()
        .set("fill", format!("#{}", qr_color))
        .set("shape-rendering", "crispEdges");

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
    std::fs::write(filename, document.to_string())?;

    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    // Set better defaults for GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 800.0])
            .with_min_inner_size([600.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "🔥 QR Forge - QR Code Generator",
        options,
        Box::new(|cc| {
            // Setup better fonts if needed
            Box::new(QrForgeGui::new(cc))
        }),
    )
}
