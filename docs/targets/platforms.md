# Platforms

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Supported Platforms & Targets

## 🖥️ Desktop (Native)

- **Linux**: x86_64, ARM64 (Primary Development Platform)
- **macOS**: Apple Silicon (M1/M2/M3), Intel
- **Windows**: x86_64, ARM64

## 🌐 Web (WASM)

- **Browsers**: Chrome, Firefox, Safari, Edge
- **Runtime**: WebAssembly (wasm32-unknown-unknown)
- **Frameworks**: Dioxus, Leptos (Planned)
- **Features**: DOM Access, WebGL/WebGPU, Canvas

## 📱 Mobile (via WASM/Native)

- **Android**: WebView integration, Native API Bridge (Planned)
- **iOS**: WKWebView integration, Native API Bridge (Planned)

## 🔌 Embedded (Microcontrollers)

- **Espressif**: ESP32, ESP32-S3, ESP32-C3
- **Raspberry Pi**: RP2040 (Pico)
- **STMicroelectronics**: STM32 Series
- **Arduino**: AVR, ARM based boards
- **Nordic**: nRF52 Series (Bluetooth LE)

## ☁️ Cloud & Serverless

- **AWS**: Lambda, Fargate
- **Google Cloud**: Cloud Run, Functions
- **Azure**: Functions, App Service
- **Edge**: Cloudflare Workers, Vercel

## 📦 Native Package Support

- **Rust**: Tokio, Axum, Serde, Dioxus, Tauri, Rustea
- **Python**: NumPy, Pandas, Django, Flask, FastAPI
- **JavaScript**: Node.js, Express.js, D3.js
- **UI**: React, Vue, Angular (via integration)

See [PLAN.md](PLAN.md) for detailed implementation roadmap of these platforms.
