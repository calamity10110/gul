# GUL Official Website

The official website for the GUL Programming Language, built with Dioxus.

## Features

- **Modern Design**: Beautiful, responsive UI with dark theme
- **Multi-Page**: Home, Learn, Docs, Playground, Community, Download
- **Interactive**: Code examples, playground, and learning resources
- **SEO Optimized**: Meta tags, semantic HTML, and performance optimized
- **Responsive**: Works on desktop, tablet, and mobile devices

## Pages

### Home (`/`)

- Hero section with value proposition
- Feature cards highlighting GUL's capabilities
- Quick start guide
- Code examples
- Call-to-action

### Learn (`/learn`)

- Learning paths for different skill levels
- Getting Started tutorials
- Multi-language integration guides
- Advanced topics

### Docs (`/docs`)

- Complete documentation
- API reference
- Syntax guide
- Standard library docs
- Examples

### Playground (`/playground`)

- Interactive code editor
- Run GUL code in the browser
- Real-time output
- Share code snippets

### Community (`/community`)

- Discord server
- Forum
- GitHub repository
- Social media links

### Download (`/download`)

- Installation instructions for all platforms
- Version information
- Changelog

## Development

### Prerequisites

- Rust (latest stable)
- Dioxus CLI

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

## Project Structure

```
web/
├── src/
│   └── main.rs          # Main application code
├── public/
│   ├── index.html       # HTML template
│   └── style.css        # Styles
├── assets/              # Static assets
├── Cargo.toml           # Rust dependencies
├── Dioxus.toml          # Dioxus configuration
└── README.md            # This file
```

## Technologies

- **Dioxus 0.6**: Modern Rust UI framework
- **Dioxus Router**: Client-side routing
- **CSS3**: Modern styling with animations
- **Google Fonts**: Inter and Fira Code fonts

## Deployment

The website can be deployed to any static hosting service:

- **Netlify**: Connect GitHub repo, build command: `dx build --release`
- **Vercel**: Similar to Netlify
- **GitHub Pages**: Use GitHub Actions
- **Cloudflare Pages**: Direct integration

## Contributing

Contributions are welcome! Please see the main GUL repository for contribution guidelines.

## License

Same as the main GUL project.

## Version

Current version: 0.11.0
