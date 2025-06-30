use eframe::egui;
use qrcode::{QrCode, EcLevel};
use std::path::PathBuf;

pub struct QrForgeGui {
    url: String,
    output_name: String,
    size: u32,
    margin: u32,
    error_correction: ErrorCorrectionLevel,
    format: OutputFormat,
    qr_color: [u8; 3],
    background_color: [u8; 3],
    status_message: String,
    generated_file_path: Option<PathBuf>,
    qr_preview: Option<egui::ColorImage>,
}

#[derive(Clone, Copy, PartialEq)]
enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl ErrorCorrectionLevel {
    fn to_ec_level(self) -> EcLevel {
        match self {
            ErrorCorrectionLevel::Low => EcLevel::L,
            ErrorCorrectionLevel::Medium => EcLevel::M,
            ErrorCorrectionLevel::Quartile => EcLevel::Q,
            ErrorCorrectionLevel::High => EcLevel::H,
        }
    }

    fn to_string(self) -> &'static str {
        match self {
            ErrorCorrectionLevel::Low => "L (Low)",
            ErrorCorrectionLevel::Medium => "M (Medium)",
            ErrorCorrectionLevel::Quartile => "Q (Quartile)",
            ErrorCorrectionLevel::High => "H (High)",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum OutputFormat {
    Png,
    Jpg,
    Bmp,
    Svg,
}

impl OutputFormat {
    fn to_string(self) -> &'static str {
        match self {
            OutputFormat::Png => "PNG",
            OutputFormat::Jpg => "JPG",
            OutputFormat::Bmp => "BMP",
            OutputFormat::Svg => "SVG",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Jpg => "jpg",
            OutputFormat::Bmp => "bmp",
            OutputFormat::Svg => "svg",
        }
    }
}

impl Default for QrForgeGui {
    fn default() -> Self {
        Self {
            url: "https://example.com".to_string(),
            output_name: "qrcode".to_string(),
            size: 800,
            margin: 4,
            error_correction: ErrorCorrectionLevel::High,
            format: OutputFormat::Png,
            qr_color: [0, 0, 0],
            background_color: [255, 255, 255],
            status_message: "Ready to generate QR codes!".to_string(),
            generated_file_path: None,
            qr_preview: None,
        }
    }
}

impl eframe::App for QrForgeGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔥 QR Forge - QR Code Generator");
            ui.separator();

            // URL Input
            ui.horizontal(|ui| {
                ui.label("🌐 URL:");
                ui.text_edit_singleline(&mut self.url);
            });

            ui.add_space(10.0);

            // Output settings
            ui.horizontal(|ui| {
                ui.label("📁 Output name:");
                ui.text_edit_singleline(&mut self.output_name);
            });

            ui.horizontal(|ui| {
                ui.label("📏 Size (pixels):");
                ui.add(egui::Slider::new(&mut self.size, 100..=4000).text("px"));
            });

            ui.horizontal(|ui| {
                ui.label("📐 Margin (modules):");
                ui.add(egui::Slider::new(&mut self.margin, 0..=10).text("modules"));
            });

            ui.add_space(10.0);

            // Format selection
            ui.horizontal(|ui| {
                ui.label("📄 Format:");
                egui::ComboBox::from_id_source("format_combo")
                    .selected_text(self.format.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.format, OutputFormat::Png, "PNG");
                        ui.selectable_value(&mut self.format, OutputFormat::Jpg, "JPG");
                        ui.selectable_value(&mut self.format, OutputFormat::Bmp, "BMP");
                        ui.selectable_value(&mut self.format, OutputFormat::Svg, "SVG");
                    });
            });

            // Error correction
            ui.horizontal(|ui| {
                ui.label("🔧 Error Correction:");
                egui::ComboBox::from_id_source("error_correction_combo")
                    .selected_text(self.error_correction.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.error_correction, ErrorCorrectionLevel::Low, "L (Low)");
                        ui.selectable_value(&mut self.error_correction, ErrorCorrectionLevel::Medium, "M (Medium)");
                        ui.selectable_value(&mut self.error_correction, ErrorCorrectionLevel::Quartile, "Q (Quartile)");
                        ui.selectable_value(&mut self.error_correction, ErrorCorrectionLevel::High, "H (High)");
                    });
            });

            ui.add_space(10.0);

            // Color settings (only for SVG)
            if self.format == OutputFormat::Svg {
                ui.label("🎨 Colors (SVG only):");
                ui.horizontal(|ui| {
                    ui.label("QR Color:");
                    ui.color_edit_button_srgb(&mut self.qr_color);
                });
                ui.horizontal(|ui| {
                    ui.label("Background:");
                    ui.color_edit_button_srgb(&mut self.background_color);
                });
                ui.add_space(10.0);
            }

            // Generate button
            if ui.add_sized([200.0, 40.0], egui::Button::new("🚀 Generate QR Code")).clicked() {
                self.generate_qr_code();
            }

            ui.add_space(10.0);

            // Status message
            ui.horizontal(|ui| {
                ui.label("📊 Status:");
                ui.label(&self.status_message);
            });

            // Show generated file path if available
            if let Some(ref path) = self.generated_file_path {
                ui.horizontal(|ui| {
                    ui.label("📁 Saved to:");
                    if ui.link(path.to_string_lossy()).clicked() {
                        // Try to open the folder containing the file
                        if let Some(parent) = path.parent() {
                            let _ = open::that(parent);
                        }
                    }
                });

                // Open file button
                if ui.button("📂 Open File").clicked() {
                    let _ = open::that(path);
                }
            }

            ui.add_space(10.0);

            // QR Preview (for small sizes)
            if let Some(ref qr_image) = self.qr_preview {
                ui.label("👀 Preview:");
                let texture = ctx.load_texture("qr_preview", qr_image.clone(), Default::default());
                ui.add(egui::Image::new(&texture).max_size(egui::Vec2::new(200.0, 200.0)));
            }
        });
    }
}

impl QrForgeGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn generate_qr_code(&mut self) {
        // Validate URL
        let url = match self.validate_url() {
            Ok(url) => url,
            Err(e) => {
                self.status_message = format!("❌ Error: {}", e);
                return;
            }
        };

        // Generate QR code
        let qr_code = match QrCode::with_error_correction_level(&url, self.error_correction.to_ec_level()) {
            Ok(qr) => qr,
            Err(e) => {
                self.status_message = format!("❌ QR generation error: {}", e);
                return;
            }
        };

        // Generate preview
        self.generate_preview(&qr_code);

        // Save file
        let filename = format!("{}.{}", self.output_name, self.format.extension());
        
        match self.format {
            OutputFormat::Svg => {
                match self.save_svg(&qr_code, &filename) {
                    Ok(_) => {
                        self.status_message = format!("✅ SVG generated successfully: {}", filename);
                        self.generated_file_path = Some(PathBuf::from(&filename));
                    }
                    Err(e) => {
                        self.status_message = format!("❌ SVG save error: {}", e);
                    }
                }
            }
            _ => {
                match self.save_bitmap(&qr_code, &filename) {
                    Ok(_) => {
                        self.status_message = format!("✅ {} generated successfully: {}", self.format.to_string(), filename);
                        self.generated_file_path = Some(PathBuf::from(&filename));
                    }
                    Err(e) => {
                        self.status_message = format!("❌ {} save error: {}", self.format.to_string(), e);
                    }
                }
            }
        }
    }

    fn validate_url(&self) -> Result<String, String> {
        let url = if self.url.starts_with("http://") || self.url.starts_with("https://") {
            self.url.clone()
        } else {
            format!("https://{}", self.url)
        };

        if !url.contains('.') {
            return Err("URL must contain at least one dot".to_string());
        }

        Ok(url)
    }

    fn generate_preview(&mut self, qr_code: &QrCode) {
        // Generate a small preview image
        let preview_size = 200;
        let qr_width = qr_code.width();
        let total_modules = qr_width + (self.margin as usize * 2);
        let module_size = preview_size / total_modules;
        let actual_size = module_size * total_modules;

        let mut pixels = vec![255u8; actual_size * actual_size * 3]; // RGB

        // Draw QR code
        for y in 0..qr_width {
            for x in 0..qr_width {
                if qr_code[(x, y)] == qrcode::Color::Dark {
                    let start_x = (x + self.margin as usize) * module_size;
                    let start_y = (y + self.margin as usize) * module_size;

                    for dy in 0..module_size {
                        for dx in 0..module_size {
                            let px = start_x + dx;
                            let py = start_y + dy;
                            
                            if px < actual_size && py < actual_size {
                                let idx = (py * actual_size + px) * 3;
                                if idx + 2 < pixels.len() {
                                    pixels[idx] = 0;     // R
                                    pixels[idx + 1] = 0; // G
                                    pixels[idx + 2] = 0; // B
                                }
                            }
                        }
                    }
                }
            }
        }

        self.qr_preview = Some(egui::ColorImage::from_rgb(
            [actual_size, actual_size],
            &pixels,
        ));
    }

    fn save_svg(&self, qr_code: &QrCode, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        crate::generate_svg_qr(
            qr_code,
            filename,
            self.size,
            self.margin,
            &format!("{:02x}{:02x}{:02x}", self.qr_color[0], self.qr_color[1], self.qr_color[2]),
            &format!("{:02x}{:02x}{:02x}", self.background_color[0], self.background_color[1], self.background_color[2]),
        )
    }

    fn save_bitmap(&self, qr_code: &QrCode, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let image = crate::generate_high_quality_image(qr_code, self.size, self.margin)?;
        
        match self.format {
            OutputFormat::Png => {
                image.save_with_format(filename, image::ImageFormat::Png)?;
            }
            OutputFormat::Jpg => {
                image.save_with_format(filename, image::ImageFormat::Jpeg)?;
            }
            OutputFormat::Bmp => {
                image.save_with_format(filename, image::ImageFormat::Bmp)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "QR Forge",
        options,
        Box::new(|cc| Box::new(QrForgeGui::new(cc))),
    )
}
