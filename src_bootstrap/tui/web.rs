// GUL TUI Web Module
// Ratzilla-powered WebAssembly TUI
// For web deployment of the GUL IDE

#[cfg(feature = "web-tui")]
pub mod web_tui {
    //! Web TUI implementation using Ratzilla
    //!
    //! This module enables running the GUL TUI IDE in a web browser
    //! via WebAssembly compilation.

    /// Web application configuration
    #[derive(Debug, Clone)]
    pub struct WebConfig {
        /// Canvas element ID
        pub canvas_id: String,
        /// Initial width
        pub width: u32,
        /// Initial height
        pub height: u32,
        /// Font family
        pub font_family: String,
        /// Font size in pixels
        pub font_size: u32,
        /// Enable touch support
        pub touch_enabled: bool,
    }

    impl Default for WebConfig {
        fn default() -> Self {
            WebConfig {
                canvas_id: "gul-tui".to_string(),
                width: 800,
                height: 600,
                font_family: "monospace".to_string(),
                font_size: 14,
                touch_enabled: true,
            }
        }
    }

    /// Web TUI state
    pub struct WebTui {
        config: WebConfig,
        // When ratzilla is integrated:
        // terminal: ratzilla::Terminal,
    }

    impl WebTui {
        /// Create new web TUI instance
        pub fn new(config: WebConfig) -> Self {
            WebTui { config }
        }

        /// Get configuration
        pub fn config(&self) -> &WebConfig {
            &self.config
        }
    }

    /// JavaScript bindings for web events
    #[cfg(target_arch = "wasm32")]
    pub mod js_bindings {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            pub fn log(s: &str);

            #[wasm_bindgen(js_namespace = console)]
            pub fn error(s: &str);
        }

        /// Log to browser console
        pub fn console_log(msg: &str) {
            log(msg);
        }

        /// Log error to browser console
        pub fn console_error(msg: &str) {
            error(msg);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub mod js_bindings {
        /// Stub for non-WASM builds
        pub fn console_log(msg: &str) {
            println!("{}", msg);
        }

        /// Stub for non-WASM builds
        pub fn console_error(msg: &str) {
            eprintln!("{}", msg);
        }
    }
}

/// Build configuration for WASM output
#[derive(Debug, Clone)]
pub struct WasmBuildConfig {
    /// Output directory
    pub out_dir: String,
    /// Package name
    pub package_name: String,
    /// Enable source maps
    pub source_maps: bool,
    /// Optimization level (0-3)
    pub opt_level: u8,
    /// Enable wasm-opt
    pub wasm_opt: bool,
}

impl Default for WasmBuildConfig {
    fn default() -> Self {
        WasmBuildConfig {
            out_dir: "dist".to_string(),
            package_name: "gul-ide".to_string(),
            source_maps: true,
            opt_level: 2,
            wasm_opt: true,
        }
    }
}

impl WasmBuildConfig {
    /// Generate wasm-pack build command
    pub fn wasm_pack_command(&self) -> String {
        format!(
            "wasm-pack build --target web --out-dir {} --{}",
            self.out_dir,
            if self.opt_level == 0 {
                "dev"
            } else {
                "release"
            }
        )
    }

    /// Generate index.html for hosting
    pub fn generate_index_html(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GUL IDE</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            background: #1e1e2e;
            overflow: hidden;
        }}
        #gul-tui {{
            position: fixed;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
        }}
        .loading {{
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: #cdd6f4;
            font-family: monospace;
            font-size: 18px;
        }}
        .loading .spinner {{
            animation: spin 1s linear infinite;
            display: inline-block;
        }}
        @keyframes spin {{
            from {{ transform: rotate(0deg); }}
            to {{ transform: rotate(360deg); }}
        }}
    </style>
</head>
<body>
    <div class="loading" id="loading">
        <span class="spinner">⠋</span> Loading GUL IDE...
    </div>
    <canvas id="gul-tui"></canvas>
    
    <script type="module">
        import init, {{ run_ide }} from './{pkg}/{pkg}.js';
        
        async function main() {{
            try {{
                await init();
                document.getElementById('loading').style.display = 'none';
                run_ide('gul-tui');
            }} catch (e) {{
                document.getElementById('loading').innerHTML = 
                    '❌ Failed to load: ' + e.message;
                console.error(e);
            }}
        }}
        
        main();
    </script>
</body>
</html>
"#,
            pkg = self.package_name
        )
    }
}

/// Deployment configuration
#[derive(Debug, Clone)]
pub struct DeployConfig {
    /// Deployment provider
    pub provider: DeployProvider,
    /// Project name
    pub project_name: String,
    /// Custom domain (optional)
    pub domain: Option<String>,
}

/// Supported deployment providers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeployProvider {
    /// Vercel
    Vercel,
    /// Netlify
    Netlify,
    /// GitHub Pages
    GitHubPages,
    /// Cloudflare Pages
    CloudflarePages,
    /// Custom/self-hosted
    Custom,
}

impl DeployProvider {
    /// Get deployment configuration file name
    pub fn config_file(&self) -> &'static str {
        match self {
            DeployProvider::Vercel => "vercel.json",
            DeployProvider::Netlify => "netlify.toml",
            DeployProvider::GitHubPages => ".github/workflows/deploy.yml",
            DeployProvider::CloudflarePages => "wrangler.toml",
            DeployProvider::Custom => "deploy.json",
        }
    }

    /// Generate deployment configuration
    pub fn generate_config(&self, project: &str) -> String {
        match self {
            DeployProvider::Vercel => format!(
                r#"{{
  "name": "{}",
  "builds": [
    {{ "src": "index.html", "use": "@vercel/static" }}
  ],
  "routes": [
    {{ "src": "/(.*)", "dest": "/$1" }}
  ]
}}
"#,
                project
            ),
            DeployProvider::Netlify => r#"[build]
  publish = "dist"
  command = "wasm-pack build --target web"

[[headers]]
  for = "/*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
"#
            .to_string(),
            DeployProvider::GitHubPages => r#"name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install wasm-pack
        run: cargo install wasm-pack
      - name: Build
        run: wasm-pack build --target web --out-dir dist
      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
"#
            .to_string(),
            DeployProvider::CloudflarePages => format!(
                r#"name = "{}"
compatibility_date = "2024-01-01"

[site]
bucket = "./dist"
"#,
                project
            ),
            DeployProvider::Custom => format!(
                r#"{{
  "project": "{}",
  "build": "wasm-pack build --target web",
  "output": "dist"
}}
"#,
                project
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_build_config() {
        let config = WasmBuildConfig::default();
        assert_eq!(config.out_dir, "dist");
        assert_eq!(config.opt_level, 2);
    }

    #[test]
    fn test_wasm_pack_command() {
        let config = WasmBuildConfig::default();
        let cmd = config.wasm_pack_command();
        assert!(cmd.contains("wasm-pack build"));
        assert!(cmd.contains("--release"));
    }

    #[test]
    fn test_deploy_providers() {
        assert_eq!(DeployProvider::Vercel.config_file(), "vercel.json");
        assert_eq!(DeployProvider::Netlify.config_file(), "netlify.toml");
    }

    #[test]
    fn test_generate_config() {
        let config = DeployProvider::Vercel.generate_config("gul-ide");
        assert!(config.contains("gul-ide"));
    }
}
