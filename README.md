# 🔥 QR Forge

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**QR Forge** è un potente generatore di QR code scritto in Rust che produce QR code di **altissima qualità** con supporto per formati vettoriali SVG e bitmap ad alta risoluzione.

## ✨ Caratteristiche

- 🎨 **Formati multipli**: SVG (scalabile), PNG, JPG, BMP
- 🔧 **Altamente personalizzabile**: colori, dimensioni, margini, correzione errori
- 📱 **Ottimizzato per stampa**: supporto per stampa professionale
- ⚡ **Veloce e efficiente**: scritto in Rust per massime prestazioni
- 🎯 **Qualità professionale**: correzione errori fino al livello H
- 📐 **SVG scalabile**: qualità infinita senza perdita di dettagli

## 🚀 Installazione

### Prerequisiti
- Rust 1.70 o superiore
- Cargo (incluso con Rust)

### Compilazione da sorgente
```bash
git clone https://github.com/francesco/qr-forge.git
cd qr-forge
cargo build --release
```

L'eseguibile sarà disponibile in `target/release/qr-forge`

## 📖 Utilizzo

### Sintassi base
```bash
qr-forge --url "https://example.com"
```

### Esempi

**QR code SVG scalabile:**
```bash
qr-forge --url "https://github.com" --format svg --output "github_qr"
```

**QR code con colori personalizzati:**
```bash
qr-forge --url "https://rust-lang.org" --format svg --color "ff6600" --background-color "f5f5dc"
```

**QR code ad alta risoluzione per stampa:**
```bash
qr-forge --url "https://example.com" --format png --size 2400 --margin 4
```

**QR code ottimizzato per stampa (senza bordo):**
```bash
qr-forge --url "https://example.com" --format svg --margin 0 --size 1200
```

### Parametri disponibili

| Parametro | Descrizione | Default | Esempio |
|-----------|-------------|---------|---------|
| `--url` | URL da codificare (obbligatorio) | - | `https://example.com` |
| `--output` | Nome file output (senza estensione) | `qrcode` | `my_qr` |
| `--size` | Dimensioni in pixel | `800` | `1200` |
| `--format` | Formato output | `png` | `svg`, `png`, `jpg`, `bmp` |
| `--margin` | Margine in moduli | `4` | `0`, `2`, `8` |
| `--error-correction` | Livello correzione errori | `H` | `L`, `M`, `Q`, `H` |
| `--color` | Colore QR (hex, solo SVG) | `000000` | `ff0000` |
| `--background-color` | Colore sfondo (hex, solo SVG) | `ffffff` | `f0f8ff` |

## 🎯 Casi d'uso

### 📱 **Per uso digitale**
```bash
qr-forge --url "https://mywebsite.com" --format svg --size 500
```

### 🖨️ **Per stampa professionale**
```bash
qr-forge --url "https://mywebsite.com" --format png --size 2400 --margin 2
```

### 🎨 **Per design personalizzato**
```bash
qr-forge --url "https://mywebsite.com" --format svg --color "2c3e50" --background-color "ecf0f1"
```

### 📋 **Per biglietti da visita**
```bash
qr-forge --url "https://linkedin.com/in/myprofile" --format svg --margin 1 --size 800
```

## 📊 Formati supportati

| Formato | Descrizione | Caso d'uso ideale |
|---------|-------------|-------------------|
| **SVG** | Vettoriale scalabile | Web, stampa professionale, loghi |
| **PNG** | Bitmap alta qualità | Stampa, presentazioni |
| **JPG** | Bitmap compresso | Web, condivisione |
| **BMP** | Bitmap non compresso | Elaborazione immagini |

## 🔧 Livelli di correzione errori

| Livello | Correzione | Capacità | Uso consigliato |
|---------|------------|----------|------------------|
| **L** | ~7% | Massima | Ambienti perfetti |
| **M** | ~15% | Alta | Uso generale |
| **Q** | ~25% | Media | Ambienti difficili |
| **H** | ~30% | Minima | **Stampa/Professionale** |

## 🎨 Esempi di output

### QR Code classico
```bash
qr-forge --url "https://rust-lang.org"
```
- Output: `qrcode.png` (800x800px, nero su bianco)

### QR Code per stampa
```bash
qr-forge --url "https://example.com" --format svg --margin 2 --size 1200
```
- Output: `qrcode.svg` (scalabile, ottimo per stampa)

### QR Code colorato
```bash
qr-forge --url "https://github.com" --format svg --color "0066cc" --background-color "f0f8ff"
```
- Output: `qrcode.svg` (blu su azzurro chiaro)

## 🏗️ Sviluppo

### Compilazione
```bash
cargo build
```

### Test
```bash
cargo test
```

### Compilazione ottimizzata
```bash
cargo build --release
```

## 🤝 Contribuire

I contributi sono benvenuti! Per favore:

1. Fai fork del progetto
2. Crea un branch per la tua feature (`git checkout -b feature/AmazingFeature`)
3. Committa le tue modifiche (`git commit -m 'Add AmazingFeature'`)
4. Pusha al branch (`git push origin feature/AmazingFeature`)
5. Apri una Pull Request

## 📝 Licenza

Questo progetto è rilasciato sotto licenza MIT. Vedi il file `LICENSE` per i dettagli.

## 🙏 Ringraziamenti

- [qrcode-rust](https://github.com/kennytm/qrcode-rust) - Libreria core per la generazione QR
- [image-rs](https://github.com/image-rs/image) - Elaborazione immagini
- [clap](https://github.com/clap-rs/clap) - Parsing argomenti CLI
- [svg](https://github.com/bodoni/svg) - Generazione SVG

---

**Fatto con ❤️ in Rust**
