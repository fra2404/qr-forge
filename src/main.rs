use clap::Parser;
use image::{ImageBuffer, Rgb};
use qrcode::{QrCode, EcLevel};
use std::fs;
use svg::Document;
use svg::node::element::{Rectangle, Group};

#[derive(Parser)]
#[command(name = "qr-forge")]
#[command(about = "🔥 QR Forge - Generatore di QR code di alta qualità con supporto SVG")]
#[command(version = "1.0.0")]
#[command(author = "Francesco")]
#[command(long_about = "QR Forge è un potente generatore di QR code che supporta formati SVG scalabili e bitmap ad alta risoluzione. Perfetto per uso professionale, stampa e web.")]
struct Args {
    /// URL del sito web per cui generare il QR code
    #[arg(short, long)]
    url: String,

    /// Nome del file di output (senza estensione)
    #[arg(short, long, default_value = "qrcode")]
    output: String,

    /// Dimensione del QR code in pixel (larghezza x altezza)
    #[arg(short, long, default_value = "800")]
    size: u32,

    /// Livello di correzione errori: L (basso), M (medio), Q (quartile), H (alto)
    #[arg(short, long, default_value = "H")]
    error_correction: String,

    /// Margine intorno al QR code (in moduli)
    #[arg(short, long, default_value = "4")]
    margin: u32,

    /// Formato di output: png, jpg, bmp, svg
    #[arg(short, long, default_value = "png")]
    format: String,

    /// Colore del QR code (per SVG, formato hex senza #)
    #[arg(long, default_value = "000000")]
    color: String,

    /// Colore dello sfondo (per SVG, formato hex senza #)
    #[arg(long, default_value = "ffffff")]
    background_color: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validazione URL
    let url = validate_url(&args.url)?;
    
    // Determina il livello di correzione errori
    let ec_level = match args.error_correction.to_uppercase().as_str() {
        "L" => EcLevel::L,
        "M" => EcLevel::M,
        "Q" => EcLevel::Q,
        "H" => EcLevel::H,
        _ => return Err("Livello di correzione errori non valido. Usa: L, M, Q, H".into()),
    };

    println!("🔧 Generando QR code per: {}", url);
    println!("📊 Parametri:");
    println!("   - Dimensione: {}x{} pixel", args.size, args.size);
    println!("   - Correzione errori: {}", args.error_correction);
    println!("   - Margine: {} moduli", args.margin);
    println!("   - Formato: {}", args.format);

    // Genera il QR code
    let qr_code = QrCode::with_error_correction_level(&url, ec_level)
        .map_err(|e| format!("Errore nella generazione del QR code: {}", e))?;

    // Determina il nome del file
    let filename = format!("{}.{}", args.output, args.format.to_lowercase());
    
    // Genera il file in base al formato
    match args.format.to_lowercase().as_str() {
        "svg" => {
            generate_svg_qr(&qr_code, &filename, args.size, args.margin, &args.color, &args.background_color)?;
        }
        _ => {
            // Crea l'immagine ad alta risoluzione per formati bitmap
            let image = generate_high_quality_image(&qr_code, args.size, args.margin)?;
            save_image(&image, &filename, &args.format)?;
        }
    }

    println!("✅ QR code generato con successo!");
    println!("📁 File salvato come: {}", filename);
    
    // Mostra statistiche del QR code
    print_qr_stats(&qr_code, &url);

    Ok(())
}

fn validate_url(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else {
        format!("https://{}", input)
    };

    // Validazione di base
    if !url.contains('.') {
        return Err("URL non valido: deve contenere almeno un punto".into());
    }

    Ok(url)
}

fn generate_high_quality_image(
    qr_code: &QrCode,
    size: u32,
    margin: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let qr_width = qr_code.width() as u32;
    let total_modules = qr_width + (margin * 2);
    let module_size = size / total_modules;
    let actual_size = module_size * total_modules;

    println!("📐 Dettagli tecnici:");
    println!("   - Moduli QR: {}x{}", qr_width, qr_width);
    println!("   - Moduli totali (con margine): {}x{}", total_modules, total_modules);
    println!("   - Dimensione modulo: {}x{} pixel", module_size, module_size);
    println!("   - Dimensione finale: {}x{} pixel", actual_size, actual_size);

    if module_size < 4 {
        eprintln!("⚠️  Attenzione: Dimensione modulo molto piccola ({}px). Considera di aumentare la dimensione totale.", module_size);
    }

    let mut image = ImageBuffer::new(actual_size, actual_size);

    // Riempi di bianco
    for pixel in image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Disegna il QR code
    for y in 0..qr_width {
        for x in 0..qr_width {
            if qr_code[(x as usize, y as usize)] == qrcode::Color::Dark {
                // Calcola la posizione con il margine
                let start_x = (x + margin) * module_size;
                let start_y = (y + margin) * module_size;

                // Disegna un modulo (quadrato nero)
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
            return Err(format!("Formato non supportato: {}. Usa png, jpg, bmp, o svg", format).into());
        }
    }

    // Mostra informazioni sul file
    if let Ok(metadata) = fs::metadata(filename) {
        let size_kb = metadata.len() as f64 / 1024.0;
        println!("📊 Dimensione file: {:.2} KB", size_kb);
    }

    Ok(())
}

fn print_qr_stats(qr_code: &QrCode, url: &str) {
    println!("\n📈 Statistiche QR Code:");
    println!("   - Versione: {:?}", qr_code.version());
    println!("   - Dimensione matrice: {}x{} moduli", qr_code.width(), qr_code.width());
    println!("   - Lunghezza URL: {} caratteri", url.len());
    println!("   - Livello correzione: {:?}", qr_code.error_correction_level());
    
    // Capacità massima per questa versione
    let max_capacity = get_max_capacity(qr_code.version(), qr_code.error_correction_level());
    println!("   - Capacità massima: {} caratteri", max_capacity);
    println!("   - Utilizzo capacità: {:.1}%", (url.len() as f64 / max_capacity as f64) * 100.0);
    
    println!("\n💡 Suggerimenti per l'uso:");
    println!("   - Testa il QR code con diversi lettori");
    println!("   - Assicurati che sia leggibile anche quando stampato");
    println!("   - Per stampe, usa almeno 2.5cm x 2.5cm");
    
    if url.len() > max_capacity * 80 / 100 {
        println!("   ⚠️  URL vicino al limite di capacità - considera di abbreviarlo");
    }
}

fn get_max_capacity(version: qrcode::Version, ec_level: EcLevel) -> usize {
    // Capacità approssimative per dati alfanumerici basate sulla versione QR
    // Usiamo i moduli della matrice per determinare la capacità
    let modules = match version {
        qrcode::Version::Normal(v) => (4 * v + 17) as u8,
        qrcode::Version::Micro(_) => 21, // Semplificazione per micro QR
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
        _ => 1000, // Valore di default per versioni superiori
    }
}

fn generate_svg_qr(
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

    println!("📐 Dettagli tecnici SVG:");
    println!("   - Moduli QR: {}x{}", qr_width, qr_width);
    println!("   - Moduli totali (con margine): {}x{}", total_modules, total_modules);
    println!("   - Dimensione modulo: {}x{} unità SVG", module_size, module_size);
    println!("   - Dimensione finale: {}x{} unità SVG", actual_size, actual_size);
    println!("   - Colore QR: #{}", qr_color);
    println!("   - Colore sfondo: #{}", bg_color);

    // Crea il documento SVG
    let mut document = Document::new()
        .set("viewBox", (0, 0, actual_size, actual_size))
        .set("width", actual_size)
        .set("height", actual_size)
        .set("xmlns", "http://www.w3.org/2000/svg");

    // Sfondo colorato
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", actual_size)
        .set("height", actual_size)
        .set("fill", format!("#{}", bg_color));
    
    document = document.add(background);

    // Gruppo per tutti i moduli del QR code con colore personalizzato
    let mut qr_group = Group::new()
        .set("fill", format!("#{}", qr_color))
        .set("shape-rendering", "crispEdges"); // Per bordi nitidi

    // Disegna i moduli del QR code
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

    // Salva il file SVG
    std::fs::write(filename, document.to_string())?;

    // Mostra informazioni sul file
    if let Ok(metadata) = fs::metadata(filename) {
        let size_kb = metadata.len() as f64 / 1024.0;
        println!("📊 Dimensione file SVG: {:.2} KB", size_kb);
    }

    println!("✨ QR code SVG generato! Scalabile all'infinito senza perdita di qualità.");

    Ok(())
}
