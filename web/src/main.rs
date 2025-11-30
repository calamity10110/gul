// GUL Programming Language Official Website
// Built with Dioxus - Simplified version

#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content",
                Hero {}
                Features {}
                QuickStart {}
                CallToAction {}
            }
            Footer {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header { class: "header",
            nav { class: "nav",
                div { class: "logo",
                    h1 { "GUL" }
                    span { class: "tagline", "Universal Language" }
                }
                ul { class: "nav-links",
                    li { a { href: "/", "Home" } }
                    li { a { href: "/learn", "Learn" } }
                    li { a { href: "/docs", "Docs" } }
                    li { a { href: "/playground", "Playground" } }
                    li { a { href: "/community", "Community" } }
                    li { a { href: "/download", class: "download-btn", "Download" } }
                }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-content",
                h1 { class: "hero-title",
                    "The Universal Programming Language"
                }
                p { class: "hero-subtitle",
                    "Write once, run everywhere. GUL seamlessly integrates Rust, Python, JavaScript, C, and SQL in a single, elegant syntax."
                }
                div { class: "hero-buttons",
                    a { href: "/learn", class: "btn btn-primary", "Get Started" }
                    a { href: "/playground", class: "btn btn-secondary", "Try Online" }
                }
            }
            div { class: "hero-code",
                pre { class: "code-block",
                    code {
                        r#"main:
    # Multi-language integration in one file
    
    # Rust for performance
    @rust
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2)
        }
    }
    
    # Python for data science
    @python
    import numpy as np
    data = np.array([1, 2, 3, 4, 5])
    
    # JavaScript for web
    @js
    console.log("Hello from GUL!");
"#
                    }
                }
            }
        }
    }
}

#[component]
fn Features() -> Element {
    rsx! {
        section { class: "features",
            h2 { class: "section-title", "Why Choose GUL?" }
            div { class: "features-grid",
                div { class: "feature-card",
                    div { class: "feature-icon", "🚀" }
                    h3 { class: "feature-title", "Multi-Language Integration" }
                    p { class: "feature-description",
                        "Seamlessly mix Rust, Python, JavaScript, C, and SQL in a single codebase with zero-copy data sharing."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "⚡" }
                    h3 { class: "feature-title", "High Performance" }
                    p { class: "feature-description",
                        "Compile to native code, WebAssembly, or embedded targets. Optimized for speed and efficiency."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🎯" }
                    h3 { class: "feature-title", "Type Safety" }
                    p { class: "feature-description",
                        "Advanced type system with inference, generics, and compile-time checks for bulletproof code."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🌐" }
                    h3 { class: "feature-title", "Cross-Platform" }
                    p { class: "feature-description",
                        "Build for desktop, web, mobile, and embedded systems from a single codebase."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🤖" }
                    h3 { class: "feature-title", "AI-Powered" }
                    p { class: "feature-description",
                        "Built-in AI assistant for code generation, refactoring, and intelligent suggestions."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🔧" }
                    h3 { class: "feature-title", "Modern Tooling" }
                    p { class: "feature-description",
                        "Integrated formatter, linter, debugger, and package manager for a complete development experience."
                    }
                }
            }
        }
    }
}

#[component]
fn QuickStart() -> Element {
    rsx! {
        section { class: "quick-start",
            h2 { class: "section-title", "Quick Start" }
            div { class: "quick-start-steps",
                div { class: "step",
                    div { class: "step-number", "1" }
                    h3 { class: "step-title", "Install GUL" }
                    pre { class: "code-block",
                        code { "curl -sSf https://gul-lang.org/install.sh | sh" }
                    }
                }
                div { class: "step",
                    div { class: "step-number", "2" }
                    h3 { class: "step-title", "Create a Project" }
                    pre { class: "code-block",
                        code { "gul new my-project\ncd my-project" }
                    }
                }
                div { class: "step",
                    div { class: "step-number", "3" }
                    h3 { class: "step-title", "Run Your Code" }
                    pre { class: "code-block",
                        code { "gul run main.gul" }
                    }
                }
            }
        }
    }
}

#[component]
fn CallToAction() -> Element {
    rsx! {
        section { class: "cta",
            h2 { "Ready to Get Started?" }
            p { "Join thousands of developers building the future with GUL" }
            div { class: "cta-buttons",
                a { href: "/download", class: "btn btn-large btn-primary", "Download GUL" }
                a { href: "/learn", class: "btn btn-large btn-secondary", "Read the Docs" }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "footer-content",
                div { class: "footer-section",
                    h4 { "GUL" }
                    p { "The Universal Programming Language" }
                    p { class: "version", "Version 0.11.0" }
                }
                div { class: "footer-section",
                    h4 { "Resources" }
                    ul {
                        li { a { href: "/docs", "Documentation" } }
                        li { a { href: "/learn", "Tutorials" } }
                        li { a { href: "/playground", "Playground" } }
                        li { a { href: "https://github.com/gul-lang", "GitHub" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Community" }
                    ul {
                        li { a { href: "https://discord.gg/gul", "Discord" } }
                        li { a { href: "https://reddit.com/r/gul", "Reddit" } }
                        li { a { href: "https://twitter.com/gul_lang", "Twitter" } }
                        li { a { href: "/community", "Forum" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Legal" }
                    ul {
                        li { a { href: "/privacy", "Privacy Policy" } }
                        li { a { href: "/terms", "Terms of Service" } }
                        li { a { href: "/license", "License" } }
                    }
                }
            }
            div { class: "footer-bottom",
                p { "© 2025 GUL Programming Language. All rights reserved." }
            }
        }
    }
}
