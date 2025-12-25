// GUL Programming Language Official Website with Web IDE
// Built with Dioxus 0.6

#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/docs")]
        Docs {},
        #[route("/playground")]
        Playground {},
        #[route("/blog")]
        Blog {},
        #[route("/community")]
        Community {},
        #[route("/download")]
        Download {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content",
                Outlet::<Route> {}
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
                Link { to: Route::Home {}, class: "logo-link",
                    div { class: "logo",
                        h1 { "GUL" }
                        span { class: "tagline", "Universal Language" }
                    }
                }
                ul { class: "nav-links",
                    li { Link { to: Route::Home {}, "Home" } }
                    li { Link { to: Route::Docs {}, "Docs" } }
                    li { Link { to: Route::Playground {}, class: "playground-link", "▶ Playground" } }
                    li { Link { to: Route::Blog {}, "Blog" } }
                    li { Link { to: Route::Community {}, "Community" } }
                    li { Link { to: Route::Download {}, class: "download-btn", "Download" } }
                }
            }
        }
    }
}

// ==================== HOME PAGE ====================

#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Features {}
        QuickStart {}
        CodeShowcase {}
        CallToAction {}
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-content",
                div { class: "hero-badge", "🚀 Version 0.13.0 Released" }
                h1 { class: "hero-title",
                    "The Universal Programming Language"
                }
                p { class: "hero-subtitle",
                    "Write once, run everywhere. GUL seamlessly integrates Rust, Python, JavaScript, C, and SQL in a single, elegant syntax."
                }
                div { class: "hero-buttons",
                    Link { to: Route::Playground {}, class: "btn btn-primary", "Try Online →" }
                    Link { to: Route::Docs {}, class: "btn btn-secondary", "Get Started" }
                }
            }
            div { class: "hero-code",
                pre { class: "code-block animated-border",
                    code {
                        span { class: "code-comment", "# GUL 101 - Multi-language in one file\n" }
                        span { class: "code-keyword", "@entry\n" }
                        span { class: "code-function", "main" }
                        span { class: "code-punct", ":\n" }
                        span { class: "code-indent", "    " }
                        span { class: "code-comment", "# Import packages\n" }
                        span { class: "code-indent", "    " }
                        span { class: "code-keyword", "@imp " }
                        span { class: "code-string", "std.io, std.http\n\n" }
                        span { class: "code-indent", "    " }
                        span { class: "code-comment", "# Async function\n" }
                        span { class: "code-indent", "    " }
                        span { class: "code-keyword", "async " }
                        span { class: "code-function", "fetch_data" }
                        span { class: "code-punct", "(url: " }
                        span { class: "code-type", "str" }
                        span { class: "code-punct", "):\n" }
                        span { class: "code-indent", "        " }
                        span { class: "code-keyword", "let " }
                        span { class: "code-variable", "response" }
                        span { class: "code-punct", " = " }
                        span { class: "code-keyword", "await " }
                        span { class: "code-function", "http.get" }
                        span { class: "code-punct", "(url)\n" }
                        span { class: "code-indent", "        " }
                        span { class: "code-keyword", "return " }
                        span { class: "code-variable", "response.json" }
                        span { class: "code-punct", "()\n\n" }
                        span { class: "code-indent", "    " }
                        span { class: "code-function", "io.print" }
                        span { class: "code-punct", "(" }
                        span { class: "code-string", "\"Hello from GUL!\"" }
                        span { class: "code-punct", ")\n" }
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
                FeatureCard {
                    icon: "🔗",
                    title: "Multi-Language FFI",
                    description: "Seamlessly call Rust, Python, JavaScript, C, and SQL from one codebase with zero-copy data sharing."
                }
                FeatureCard {
                    icon: "⚡",
                    title: "Blazing Fast",
                    description: "Compile to native code via LLVM, WebAssembly for browsers, or run interpreted for rapid prototyping."
                }
                FeatureCard {
                    icon: "🎯",
                    title: "Type Safe",
                    description: "Advanced type inference with gradual typing. Catch errors at compile time without annotation burden."
                }
                FeatureCard {
                    icon: "🌐",
                    title: "Cross-Platform",
                    description: "Build for desktop, web, mobile, embedded, and serverless from a single codebase."
                }
                FeatureCard {
                    icon: "🤖",
                    title: "AI-Powered",
                    description: "Built-in AI assistant for code generation, refactoring suggestions, and intelligent completions."
                }
                FeatureCard {
                    icon: "📦",
                    title: "Rich Ecosystem",
                    description: "4000+ packages. Modern tooling with formatter, linter, debugger, and package manager."
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "feature-card",
            div { class: "feature-icon", "{icon}" }
            h3 { class: "feature-title", "{title}" }
            p { class: "feature-description", "{description}" }
        }
    }
}

#[component]
fn QuickStart() -> Element {
    rsx! {
        section { class: "quick-start",
            h2 { class: "section-title", "Get Started in Seconds" }
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
fn CodeShowcase() -> Element {
    let mut selected_tab = use_signal(|| 0);

    let examples = [
        ("Hello World", r#"@entry
main:
    @imp std.io
    io.print("Hello, GUL!")
"#),
        ("HTTP Server", r#"@imp std.http

server = http.Server(port=8080)

@server.get("/")
index(request):
    return "Welcome to GUL!"

server.start()
"#),
        ("Async/Await", r#"@imp std.http, std.json

async fetch_users():
    let response = await http.get("https://api.example.com/users")
    let data = json.parse(response.body)
    return data.users

@entry
main:
    let users = await fetch_users()
    for user in users:
        print(user.name)
"#),
        ("Foreign Code", r#"@entry
main:
    # Call Python for data science
    @python {
        import numpy as np
        data = np.array([1, 2, 3, 4, 5])
        result = np.mean(data)
    }
    
    print(f"Mean: {result}")
"#),
    ];

    rsx! {
        section { class: "code-showcase",
            h2 { class: "section-title", "See GUL in Action" }
            div { class: "showcase-container",
                div { class: "showcase-tabs",
                    for (i, (name, _)) in examples.iter().enumerate() {
                        button {
                            class: if selected_tab() == i { "tab active" } else { "tab" },
                            onclick: move |_| selected_tab.set(i),
                            "{name}"
                        }
                    }
                }
                pre { class: "code-block showcase-code",
                    code { "{examples[selected_tab()].1}" }
                }
                div { class: "showcase-actions",
                    Link {
                        to: Route::Playground {},
                        class: "btn btn-primary btn-small",
                        "▶ Run in Playground"
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
            h2 { "Ready to Build Something Amazing?" }
            p { "Join thousands of developers building the future with GUL" }
            div { class: "cta-stats",
                div { class: "stat",
                    span { class: "stat-value", "4,000+" }
                    span { class: "stat-label", "Packages" }
                }
                div { class: "stat",
                    span { class: "stat-value", "10K+" }
                    span { class: "stat-label", "Developers" }
                }
                div { class: "stat",
                    span { class: "stat-value", "5" }
                    span { class: "stat-label", "FFI Languages" }
                }
            }
            div { class: "cta-buttons",
                Link { to: Route::Download {}, class: "btn btn-large btn-primary", "Download GUL" }
                Link { to: Route::Docs {}, class: "btn btn-large btn-secondary", "Read the Docs" }
            }
        }
    }
}

// ==================== PLAYGROUND/IDE PAGE ====================

#[component]
fn Playground() -> Element {
    let mut code = use_signal(|| String::from(r#"@entry
main:
    @imp std.io
    
    # Welcome to the GUL Playground!
    io.print("Hello, GUL!")
    
    # Try editing this code
    let numbers = [1, 2, 3, 4, 5]
    
    for num in numbers:
        io.print(f"Number: {{num}}")
    
    io.print("Done!")
"#));
    let mut output = use_signal(|| String::from("Click 'Run' to execute your code..."));
    let mut is_running = use_signal(|| false);

    let run_code = move |_| {
        is_running.set(true);
        // Simulate code execution
        let simulated_output = "🔧 Compiling...\n✓ Compiled successfully\n\n📤 Output:\nHello, GUL!\nNumber: 1\nNumber: 2\nNumber: 3\nNumber: 4\nNumber: 5\nDone!\n\n✅ Execution completed in 0.023s".to_string();
        output.set(simulated_output);
        is_running.set(false);
    };

    let clear_output = move |_| {
        output.set(String::from("Output cleared."));
    };

    let load_hello = move |_| {
        code.set(r#"@entry
main:
    @imp std.io
    io.print("Hello, GUL!")
"#.to_string());
    };

    let load_fibonacci = move |_| {
        code.set(r#"@entry
main:
    @imp std.io
    
    fib(n: int) -> int:
        if n <= 1:
            return n
        return fib(n - 1) + fib(n - 2)
    
    for i in range(10):
        io.print(f"fib({{i}}) = {{fib(i)}}")
"#.to_string());
    };

    let load_http = move |_| {
        code.set(r#"@imp std.http

server = http.Server(port=8080)

@server.get("/")
index(request):
    return http.Response(
        body="Hello from GUL!",
        content_type="text/plain"
    )

@server.get("/api/status")
status(request):
    return http.json_response({
        "status": "ok",
        "version": "0.13.0"
    })

server.start()
io.print("Server running on http://localhost:8080")
"#.to_string());
    };

    let current_code = code();
    let current_output = output();
    let running = is_running();

    rsx! {
        div { class: "playground-page",
            div { class: "playground-header",
                h1 { "GUL Playground" }
                p { "Write, run, and share GUL code directly in your browser" }
            }
            
            div { class: "playground-toolbar",
                div { class: "toolbar-left",
                    button {
                        class: "btn btn-run",
                        disabled: running,
                        onclick: run_code,
                        if running { "⏳ Running..." } else { "▶ Run" }
                    }
                    button { class: "btn btn-secondary btn-small", onclick: clear_output, "Clear" }
                }
                div { class: "toolbar-right",
                    button { class: "btn btn-secondary btn-small", onclick: load_hello, "Hello World" }
                    button { class: "btn btn-secondary btn-small", onclick: load_fibonacci, "Fibonacci" }
                    button { class: "btn btn-secondary btn-small", onclick: load_http, "HTTP Server" }
                    button { class: "btn btn-secondary btn-small", "📤 Share" }
                }
            }

            div { class: "playground-container",
                div { class: "editor-panel",
                    div { class: "panel-header",
                        span { class: "panel-title", "📝 main.gul" }
                        span { class: "panel-info", "GUL 0.13.0" }
                    }
                    textarea {
                        class: "code-editor",
                        spellcheck: false,
                        value: "{current_code}",
                        oninput: move |evt| code.set(evt.value())
                    }
                }
                div { class: "output-panel",
                    div { class: "panel-header",
                        span { class: "panel-title", "📤 Output" }
                    }
                    pre { class: "output",
                        code { "{current_output}" }
                    }
                }
            }

            div { class: "playground-tips",
                h3 { "💡 Tips" }
                ul {
                    li { "Use @imp to import packages (GUL 101 syntax)" }
                    li { "Click an example button above to load sample code" }
                    li { "Use @entry to mark the entry point" }
                    li { "Foreign code blocks: @python, @rust, @js" }
                }
            }
        }
    }
}

// ==================== DOCS PAGE ====================

#[component]
fn Docs() -> Element {
    rsx! {
        div { class: "docs-page",
            aside { class: "docs-sidebar",
                nav {
                    h3 { "Getting Started" }
                    ul {
                        li { a { href: "#installation", "Installation" } }
                        li { a { href: "#first-program", "First Program" } }
                        li { a { href: "#gul101-syntax", "GUL 101 Syntax" } }
                    }
                    h3 { "Language Guide" }
                    ul {
                        li { a { href: "#variables", "Variables & Types" } }
                        li { a { href: "#functions", "Functions" } }
                        li { a { href: "#control-flow", "Control Flow" } }
                        li { a { href: "#async", "Async/Await" } }
                    }
                    h3 { "FFI Integration" }
                    ul {
                        li { a { href: "#rust-ffi", "Rust FFI" } }
                        li { a { href: "#python-ffi", "Python FFI" } }
                        li { a { href: "#js-ffi", "JavaScript FFI" } }
                    }
                    h3 { "Standard Library" }
                    ul {
                        li { a { href: "#std-io", "std.io" } }
                        li { a { href: "#std-http", "std.http" } }
                        li { a { href: "#std-json", "std.json" } }
                        li { a { href: "#std-database", "std.database" } }
                    }
                }
            }
            
            div { class: "docs-content",
                section { id: "installation",
                    h1 { "Getting Started with GUL" }
                    h2 { "Installation" }
                    p { "Install GUL using the official installer:" }
                    pre { class: "code-block",
                        code { "curl -sSf https://gul-lang.org/install.sh | sh" }
                    }
                    p { "Or using package managers:" }
                    pre { class: "code-block",
                        code { 
                            "# macOS\nbrew install gul-lang\n\n"
                            "# Windows\nwinget install gul-lang\n\n"
                            "# Cargo\ncargo install gul"
                        }
                    }
                }

                section { id: "first-program",
                    h2 { "Your First Program" }
                    p { "Create a file called " code { "main.gul" } " with the following content:" }
                    pre { class: "code-block",
                        code {
                            "@entry\n"
                            "main:\n"
                            "    @imp std.io\n"
                            "    io.print(\"Hello, World!\")\n"
                        }
                    }
                    p { "Run it with:" }
                    pre { class: "code-block",
                        code { "gul run main.gul" }
                    }
                }

                section { id: "gul101-syntax",
                    h2 { "GUL 101 Syntax" }
                    p { "GUL 101 is the modern, clean syntax for GUL. Key features:" }
                    ul {
                        li { code { "@imp" } " - Import packages (replaces " code { "import" } ")" }
                        li { code { "@entry" } " - Mark entry point function" }
                        li { code { "let" } " - Immutable variable binding" }
                        li { code { "var" } " - Mutable variable binding" }
                        li { code { "fn_name():" } " - Function definition (no " code { "fn" } " keyword needed)" }
                    }
                    pre { class: "code-block",
                        code {
                            "# GUL 101 example\n"
                            "@imp std.io, std.math\n\n"
                            "calculate(x: int, y: int) -> int:\n"
                            "    return math.pow(x, y)\n\n"
                            "@entry\n"
                            "main:\n"
                            "    let res = calculate(2, 10)\n"
                            "    io.print(f\"2^10 = {{res}}\")\n"
                        }
                    }
                }

                div { class: "docs-nav",
                    Link { to: Route::Playground {}, class: "btn btn-primary", "Try in Playground →" }
                }
            }
        }
    }
}

// ==================== BLOG PAGE ====================

#[component]
fn Blog() -> Element {
    rsx! {
        div { class: "page-container blog-page",
            h1 { "Blog" }
            p { class: "page-subtitle", "Latest news and updates from the GUL team" }
            
            div { class: "blog-list",
                BlogPost {
                    title: "GUL v0.13.0 Released - Web IDE & Enhanced FFI",
                    date: "December 19, 2024",
                    excerpt: "We're excited to announce GUL v0.13.0! This release includes the new Web IDE/Playground, improved FFI performance, and over 500 new packages.",
                    is_featured: true
                }
                BlogPost {
                    title: "GUL 101 Syntax - A Cleaner Way to Write Code",
                    date: "December 10, 2024",
                    excerpt: "Introducing GUL 101, our new streamlined syntax that makes code more readable while maintaining full backwards compatibility."
                }
                BlogPost {
                    title: "Building Web Apps with GUL + Dioxus",
                    date: "November 28, 2024",
                    excerpt: "Learn how to build modern, reactive web applications using GUL's seamless Rust integration with the Dioxus framework."
                }
                BlogPost {
                    title: "GUL's Multi-Language FFI: How It Works",
                    date: "November 15, 2024",
                    excerpt: "Deep dive into GUL's foreign function interface and how it achieves zero-copy interop between Rust, Python, JavaScript, and C."
                }
            }
        }
    }
}

#[component]
fn BlogPost(title: &'static str, date: &'static str, excerpt: &'static str, is_featured: Option<bool>) -> Element {
    let featured = is_featured.unwrap_or(false);
    rsx! {
        article { class: if featured { "blog-post featured" } else { "blog-post" },
            if featured {
                span { class: "featured-badge", "⭐ Featured" }
            }
            h2 { "{title}" }
            span { class: "date", "{date}" }
            p { "{excerpt}" }
            a { href: "#", class: "read-more", "Read more →" }
        }
    }
}

// ==================== COMMUNITY PAGE ====================

#[component]
fn Community() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { "Join the Community" }
            p { class: "page-subtitle", "Connect with GUL developers worldwide" }
            
            div { class: "community-grid",
                CommunityCard {
                    icon: "💬",
                    title: "Discord",
                    description: "Real-time chat with GUL developers",
                    link: "https://discord.gg/gul",
                    members: "5,000+"
                }
                CommunityCard {
                    icon: "🐙",
                    title: "GitHub",
                    description: "Contribute to GUL's development",
                    link: "https://github.com/gul-lang/gul",
                    members: "2,500+ stars"
                }
                CommunityCard {
                    icon: "📢",
                    title: "Twitter/X",
                    description: "Follow for updates and announcements",
                    link: "https://twitter.com/gul_lang",
                    members: "8,000+"
                }
                CommunityCard {
                    icon: "📺",
                    title: "YouTube",
                    description: "Tutorials and conference talks",
                    link: "https://youtube.com/@gul-lang",
                    members: "1,200+"
                }
            }

            section { class: "contribute-section",
                h2 { "How to Contribute" }
                div { class: "contribute-grid",
                    div { class: "contribute-card",
                        h3 { "🐛 Report Bugs" }
                        p { "Found an issue? Report it on GitHub Issues" }
                    }
                    div { class: "contribute-card",
                        h3 { "📝 Improve Docs" }
                        p { "Help make our documentation better" }
                    }
                    div { class: "contribute-card",
                        h3 { "📦 Create Packages" }
                        p { "Build and publish packages for the ecosystem" }
                    }
                    div { class: "contribute-card",
                        h3 { "💡 Share Ideas" }
                        p { "Propose features in GitHub Discussions" }
                    }
                }
            }
        }
    }
}

#[component]
fn CommunityCard(icon: &'static str, title: &'static str, description: &'static str, link: &'static str, members: &'static str) -> Element {
    rsx! {
        a { href: "{link}", target: "_blank", class: "community-card",
            div { class: "community-icon", "{icon}" }
            h3 { "{title}" }
            p { "{description}" }
            span { class: "member-count", "{members}" }
        }
    }
}

// ==================== DOWNLOAD PAGE ====================

#[component]
fn Download() -> Element {
    rsx! {
        div { class: "page-container download-page",
            h1 { "Download GUL" }
            p { class: "page-subtitle", "Get the latest version of GUL for your platform" }
            
            div { class: "version-banner",
                span { class: "current-version", "Current Version: 0.13.0" }
                span { class: "release-date", "Released: December 19, 2024" }
            }

            div { class: "download-options",
                div { class: "download-card",
                    div { class: "platform-icon", "🐧" }
                    h3 { "Linux" }
                    pre { class: "code-block",
                        code { "curl -sSf https://gul-lang.org/install.sh | sh" }
                    }
                    p { class: "platform-note", "Supports x86_64, ARM64, RISC-V" }
                }
                div { class: "download-card",
                    div { class: "platform-icon", "🍎" }
                    h3 { "macOS" }
                    pre { class: "code-block",
                        code { "brew install gul-lang" }
                    }
                    p { class: "platform-note", "Intel & Apple Silicon" }
                }
                div { class: "download-card",
                    div { class: "platform-icon", "🪟" }
                    h3 { "Windows" }
                    pre { class: "code-block",
                        code { "winget install gul-lang" }
                    }
                    p { class: "platform-note", "Windows 10/11 x64" }
                }
            }

            section { class: "alternative-installs",
                h2 { "Alternative Installation Methods" }
                div { class: "install-grid",
                    div { class: "install-option",
                        h4 { "Cargo (Rust)" }
                        pre { class: "code-block",
                            code { "cargo install gul" }
                        }
                    }
                    div { class: "install-option",
                        h4 { "Docker" }
                        pre { class: "code-block",
                            code { "docker pull ghcr.io/gul-lang/gul:latest" }
                        }
                    }
                    div { class: "install-option",
                        h4 { "From Source" }
                        pre { class: "code-block",
                            code { "git clone https://github.com/gul-lang/gul\ncd gul && cargo build --release" }
                        }
                    }
                }
            }

            section { class: "whats-new",
                h2 { "What's New in 0.13.0" }
                ul { class: "changelog",
                    li { "✨ New Web IDE/Playground" }
                    li { "🚀 40% faster FFI calls" }
                    li { "📦 500+ new packages" }
                    li { "🔧 GUL 101 syntax refinements" }
                    li { "🐛 Bug fixes and stability improvements" }
                }
            }
        }
    }
}

// ==================== ERROR PAGE ====================

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page-container error-page",
            div { class: "error-content",
                h1 { class: "error-code", "404" }
                h2 { "Page Not Found" }
                p { "The page " code { "/{route.join(\"/\")}" } " doesn't exist." }
                Link { to: Route::Home {}, class: "btn btn-primary", "← Back to Home" }
            }
        }
    }
}

// ==================== FOOTER ====================

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "footer-content",
                div { class: "footer-section",
                    h4 { "GUL" }
                    p { "The Universal Programming Language" }
                    p { class: "version", "Version 0.13.0" }
                }
                div { class: "footer-section",
                    h4 { "Resources" }
                    ul {
                        li { Link { to: Route::Docs {}, "Documentation" } }
                        li { Link { to: Route::Playground {}, "Playground" } }
                        li { a { href: "https://github.com/gul-lang", target: "_blank", "GitHub" } }
                        li { a { href: "#", "API Reference" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Community" }
                    ul {
                        li { a { href: "https://discord.gg/gul", target: "_blank", "Discord" } }
                        li { a { href: "https://twitter.com/gul_lang", target: "_blank", "Twitter" } }
                        li { a { href: "https://reddit.com/r/gul", target: "_blank", "Reddit" } }
                        li { Link { to: Route::Community {}, "Forum" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Legal" }
                    ul {
                        li { a { href: "#", "Privacy Policy" } }
                        li { a { href: "#", "Terms of Service" } }
                        li { a { href: "#", "MIT License" } }
                    }
                }
            }
            div { class: "footer-bottom",
                p { "© 2024 GUL Programming Language. Built with ❤️ using Dioxus." }
            }
        }
    }
}
