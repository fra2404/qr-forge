# 🚀 Guida alla Distribuzione su GitHub

Questa guida ti spiega passo dopo passo come pubblicare QR Forge su GitHub con release automatiche.

## 📋 Preparazione

### 1. Crea un repository su GitHub

1. Vai su [GitHub.com](https://github.com) e fai login
2. Clicca su "New repository" (+ in alto a destra)
3. Configura il repository:
   - **Nome**: `qr-forge`
   - **Descrizione**: `High-quality QR code generator with SVG and PNG support`
   - **Visibilità**: Public (per release pubbliche) o Private
   - **NON** inizializzare con README (il nostro progetto ne ha già uno)

### 2. Aggiorna i link nel progetto

Prima di pubblicare, sostituisci `fra2404` con il tuo username GitHub in questi file:

**README.md:**

```bash
sed -i '' 's/fra2404/tuo-username-github/g' README.md
```

**Cargo.toml:**

```bash
sed -i '' 's/fra2404/tuo-username-github/g' Cargo.toml
```

Oppure modifica manualmente questi file sostituendo `fra2404` con il tuo username.

### 3. Configura il repository locale

```bash
# Se non hai ancora impostato origin
git remote add origin https://github.com/TUO-USERNAME/qr-forge.git

# Se origin esiste già, aggiornalo
git remote set-url origin https://github.com/TUO-USERNAME/qr-forge.git
```

## 🚀 Pubblicazione

### 1. Push iniziale

```bash
# Push del codice
git push -u origin main
```

### 2. Creazione della prima release

```bash
# Crea e push del tag per la prima release
git tag v1.0.0
git push origin v1.0.0
```

🎉 **Questo attiverà automaticamente:**

- Build per Linux, macOS e Windows
- Creazione degli archivi di distribuzione
- Pubblicazione della release su GitHub
- Upload di tutti i file binari

## 📦 Cosa viene distribuito automaticamente

Quando crei un tag `v*.*.*`, GitHub Actions genererà:

### Per ogni piattaforma:

- **CLI binaries**: `qr-forge-cli-{platform}.{tar.gz|zip}`
- **GUI binaries**: `qr-forge-gui-{platform}.{tar.gz|zip}`

### Speciale per macOS:

- **App Bundle**: `QR-Forge-GUI-macos.dmg` (file .dmg con l'app)

### Struttura della release:

```
Release v1.0.0
├── qr-forge-cli-linux.tar.gz      # CLI per Linux
├── qr-forge-cli-macos.tar.gz      # CLI per macOS
├── qr-forge-cli-windows.zip       # CLI per Windows
├── qr-forge-gui-linux.tar.gz      # GUI per Linux
├── qr-forge-gui-macos.tar.gz      # GUI per macOS
├── qr-forge-gui-windows.zip       # GUI per Windows
└── QR-Forge-GUI-macos.dmg         # App macOS completa
```

## 🔄 Release future

Per nuove release:

```bash
# Aggiorna il numero di versione in Cargo.toml
# Aggiorna CHANGELOG.md con le novità
# Commit le modifiche
git add -A
git commit -m "chore: bump version to v1.1.0"

# Crea e push del nuovo tag
git tag v1.1.0
git push origin v1.1.0
```

## 🛠️ Configurazioni avanzate

### Personalizzare le GitHub Actions

I file in `.github/workflows/` possono essere modificati per:

- Aggiungere più piattaforme di build
- Modificare i test automatici
- Cambiare il formato delle release

### Aggiungere badge al README

Sostituisci nel README.md:

- `fra2404` con il tuo username GitHub
- I badge si aggiorneranno automaticamente

### Configurare notifiche

In **Settings > Notifications** del repository puoi configurare:

- Notifiche per nuove release
- Notifiche per build fallite
- Email per contributori

## 📊 Monitoring

Dopo la pubblicazione puoi monitorare:

- **Actions tab**: Status dei build automatici
- **Releases**: Download e statistiche delle release
- **Insights**: Traffico e utilizzo del repository
- **Issues**: Bug report e richieste di feature

## 🎯 Checklist finale

Prima di pubblicare, verifica:

- [ ] Tutti i link `fra2404` sono stati sostituiti
- [ ] I test passano (`cargo test`)
- [ ] Il codice è formattato (`cargo fmt`)
- [ ] Non ci sono warning di clippy (`cargo clippy`)
- [ ] README.md è aggiornato
- [ ] CHANGELOG.md è aggiornato
- [ ] La versione in Cargo.toml è corretta

## 🆘 Risoluzione problemi

### Build fallisce su Linux

- Problema: librerie GTK mancanti
- Soluzione: le dipendenze sono installate automaticamente

### Release non viene creata

- Verifica che il tag inizi con `v` (es. `v1.0.0`)
- Controlla la tab "Actions" per errori

### App macOS non firmata

- Normale per sviluppatori individuali
- Gli utenti vedranno un avviso di sicurezza (normale)

## 🚀 Pronto per il lancio!

Una volta completati questi passaggi, il tuo progetto sarà:
✅ Pubblicato su GitHub
✅ Con release automatiche multi-piattaforma  
✅ Con documentazione completa
✅ Pronto per contributi della community

**Buona fortuna con QR Forge! 🔥**
