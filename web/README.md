# GUL Official Website & Web IDE

The official website for the GUL Programming Language, featuring an interactive playground/IDE, built with Dioxus 0.6.

## ✨ Features

- **Modern Design**: Beautiful, responsive dark theme with glassmorphism effects
- **Interactive Playground**: Write, run, and share GUL code in the browser
- **Multi-Page**: Home, Docs, Playground, Blog, Community, Download
- **Code Highlighting**: Syntax-highlighted code examples with GUL 101 syntax
- **SEO Optimized**: Meta tags, semantic HTML, and performance optimized
- **Responsive**: Works on desktop, tablet, and mobile devices

## 📄 Pages

### Home (`/`)

- Hero section with animated code example
- Feature cards highlighting GUL's capabilities
- Quick start guide with installation steps
- Interactive code showcase with multiple examples
- Statistics and call-to-action

### Playground (`/playground`)

- Full-featured code editor
- Example code templates
- Simulated code execution
- Share functionality
- Keyboard shortcuts

### Docs (`/docs`)

- Getting started guide
- GUL 101 syntax reference
- FFI integration guides
- Standard library documentation
- Sidebar navigation

### Blog (`/blog`)

- Latest news and updates
- Featured post highlighting
- Release announcements

### Community (`/community`)

- Discord, GitHub, Twitter links
- Contribution guidelines
- Community statistics

### Download (`/download`)

- Platform-specific installation
- Alternative methods (Cargo, Docker, source)
- Changelog and what's new

## 🛠️ Development

### Prerequisites

- Rust (latest stable)
- Dioxus CLI (`dx`)

### Install Dioxus CLI

```bash
cargo install dioxus-cli
```

### Run Development Server

```bash
cd web
dx serve
```

The website will be available at `http://localhost:8080`

### Build for Production

```bash
dx build --release
```

The built files will be in `dist/`

## 📁 Project Structure

```text
web/
├── src/
│   └── main.rs          # Main Dioxus application
├── public/
│   ├── index.html       # HTML template
│   └── style.css        # Complete CSS (1100+ lines)
├── assets/              # Static assets (images, fonts)
├── Cargo.toml           # Rust dependencies
├── Dioxus.toml          # Dioxus configuration
└── README.md            # This file
```

## 🎨 Design System

### Color Palette

| Color      | Hex       | Usage                   |
| ---------- | --------- | ----------------------- |
| Primary    | `#6366f1` | Buttons, links, accents |
| Secondary  | `#8b5cf6` | Gradients, highlights   |
| Accent     | `#ec4899` | Featured items, CTAs    |
| Background | `#0f172a` | Page background         |
| Surface    | `#1e293b` | Cards, panels           |
| Text       | `#f1f5f9` | Primary text            |

### Typography

- **Headings**: Inter (900, 800, 700)
- **Body**: Inter (400, 500)
- **Code**: Fira Code (400, 500)

### Effects

- Glassmorphism with `backdrop-filter: blur()`
- Gradient text with `-webkit-background-clip`
- Animated border glow effects
- Smooth hover transitions

## 🔧 Technologies

- **Dioxus 0.6**: Modern Rust UI framework for web
- **Dioxus Router**: Client-side navigation
- **WebAssembly**: Compiled Rust running in browser
- **CSS3**: Modern styling with custom properties
- **Google Fonts**: Inter and Fira Code

## 🚀 Deployment

The website can be deployed to any static hosting service:

### Netlify

```bash
# Build command
dx build --release

# Publish directory
dist/
```

### Vercel

Similar configuration to Netlify

### GitHub Pages

Use GitHub Actions with the `dx build --release` command

### Cloudflare Pages

Direct GitHub integration with `dx build --release`

## 📊 Performance

- Optimized WebAssembly bundle with LTO
- Lazy-loaded components
- CSS-only animations (no JS overhead)
- Efficient virtual DOM updates

## 🔜 Roadmap

- [ ] Real WASM-based GUL interpreter for playground
- [ ] Code sharing with unique URLs
- [ ] Dark/light theme toggle
- [ ] Search functionality
- [ ] User accounts for saved snippets
- [ ] API documentation generator

## 📝 Contributing

Contributions are welcome! See the main GUL repository for contribution guidelines.

## 📄 License

Same as the main GUL project (MIT).

## 🏷️ Version

Website Version: **0.13.0** (synced with GUL)
