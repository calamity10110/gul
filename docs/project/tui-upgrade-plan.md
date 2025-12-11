# GUL TUI IDE Upgrade Plan

**Date**: 2025-12-10  
**Status**: 📋 Planning  
**Target**: v0.15.0

---

## 📊 Framework Analysis

### Reviewed Frameworks

| Framework        | Language | Stars | Purpose          | Key Features                      |
| ---------------- | -------- | ----- | ---------------- | --------------------------------- |
| **Ratatui**      | Rust     | 11.8k | TUI rendering    | Widgets, layouts, immediate-mode  |
| **Bubbletea**    | Go       | 17k+  | TUI framework    | Elm architecture, component-based |
| **Ultraviolet**  | Go       | New   | TUI primitives   | Cursed renderer, high performance |
| **Glow**         | Go       | 15k+  | Markdown CLI     | Beautiful rendering, paging       |
| **Freeze**       | Go       | 3k+   | Code screenshots | Syntax highlighting, themes       |
| **Harmonica**    | Go       | 1.6k  | Animations       | Physics-based spring animations   |
| **Soft-Serve**   | Go       | 5k+   | Git server TUI   | Full-featured TUI application     |
| **Ratzilla**     | Rust     | 1k+   | Web TUI          | Ratatui → WebAssembly             |
| **egui_ratatui** | Rust     | 500+  | GUI + TUI        | Ratatui in egui, WASM support     |

### Additional Widget Libraries

| Library                  | Purpose            | Key Widgets                          |
| ------------------------ | ------------------ | ------------------------------------ |
| **tui-widgets**          | Widget collection  | Big text, popup, prompts, scrollview |
| **ratatui-image**        | Image rendering    | Sixel, iTerm2, Kitty protocols       |
| **throbber-widgets-tui** | Loading indicators | Spinners, throbbers, progress        |
| **tui-menu**             | Menu system        | Nested menus, subgroups              |

### Dashboard & Fuzzy Finder Inspirations

| Library             | Language | Purpose       | Key Features                             |
| ------------------- | -------- | ------------- | ---------------------------------------- |
| **television**      | Rust     | Fuzzy finder  | Channels, previews, nucleo matching      |
| **termui**          | Go       | Dashboards    | BarChart, Gauge, Image, Sparkline, Table |
| **blessed-contrib** | Node.js  | Dashboards    | Line/Bar charts, Map, LCD, Donut, Log    |
| **blessed**         | Node.js  | TUI framework | Comprehensive widget system              |

### Framework Recommendations

#### Primary: **Ratatui** (Rust)

- ✅ Already mentioned in current TUI IDE
- ✅ Native Rust (matches GUL compiler)
- ✅ Active community (261 contributors)
- ✅ Rich widget library
- ✅ 110 releases, stable API

#### Secondary: **Ratzilla** (Web TUI)

- ✅ Ratatui → WebAssembly
- ✅ Web deployment capability
- ✅ Same widget API
- ✅ Enables browser-based IDE

#### Widget Extensions

- **tui-widgets**: Big text banners, popups, prompts
- **ratatui-image**: Image rendering in terminal
- **throbber-widgets-tui**: Loading spinners
- **tui-menu**: Dropdown and context menus

#### Inspiration: **Charmbracelet Stack** (Go)

- Design patterns from Bubbletea
- Animation ideas from Harmonica
- Rendering concepts from Ultraviolet
- Screenshot capability from Freeze

---

## 🎛️ GUL Native UI Components (`^&^`)

### Complete Widget Reference

GUL provides native TUI widgets via the `^&^` syntax or `ui.print()` function.

#### Basic Input Widgets

| Component  | Description      | Properties                                      |
| ---------- | ---------------- | ----------------------------------------------- |
| `button`   | Clickable button | `text`, `color`, `disabled`, `onClick`          |
| `input`    | Text input field | `placeholder`, `value`, `password`, `maxLength` |
| `textarea` | Multi-line text  | `value`, `rows`, `cols`, `scroll`               |
| `checkbox` | Toggle checkbox  | `checked`, `label`, `disabled`                  |
| `radio`    | Radio button     | `selected`, `options`, `name`                   |
| `select`   | Dropdown select  | `options`, `value`, `placeholder`               |
| `slider`   | Value slider     | `min`, `max`, `value`, `step`                   |
| `toggle`   | On/off switch    | `on`, `label`, `color`                          |

#### Display Widgets

| Component   | Description      | Properties                           |
| ----------- | ---------------- | ------------------------------------ |
| `label`     | Text label       | `text`, `style`, `wrap`              |
| `text`      | Styled text      | `content`, `bold`, `italic`, `color` |
| `bigtext`   | Large ASCII text | `text`, `font`, `color`              |
| `paragraph` | Text paragraph   | `content`, `scroll`, `wrap`          |
| `sparkline` | Mini chart       | `data`, `color`, `max`               |
| `gauge`     | Progress bar     | `value`, `max`, `label`, `color`     |
| `barchart`  | Bar chart        | `data`, `labels`, `color`            |
| `canvas`    | Drawing canvas   | `width`, `height`, `points`          |

#### Container Widgets

| Component    | Description       | Properties                               |
| ------------ | ----------------- | ---------------------------------------- |
| `container`  | Generic container | `border`, `title`, `padding`             |
| `block`      | Block with border | `title`, `borders`, `style`              |
| `row`        | Horizontal layout | `gap`, `align`, `children`               |
| `column`     | Vertical layout   | `gap`, `align`, `children`               |
| `grid`       | Grid layout       | `rows`, `cols`, `gap`                    |
| `stack`      | Stacked layers    | `children`, `index`                      |
| `split`      | Split panes       | `direction`, `ratio`, `resizable`        |
| `tabs`       | Tab container     | `tabs`, `active`, `closable`             |
| `scrollview` | Scrollable area   | `content`, `scroll_x`, `scroll_y`        |
| `popup`      | Modal popup       | `title`, `content`, `visible`, `buttons` |

#### Data Widgets

| Component  | Description     | Properties                                 |
| ---------- | --------------- | ------------------------------------------ |
| `table`    | Data table      | `data`, `headers`, `selectable`, `striped` |
| `list`     | Item list       | `items`, `selected`, `multi_select`        |
| `tree`     | Tree view       | `nodes`, `expanded`, `selectable`          |
| `calendar` | Date calendar   | `date`, `range`, `marked_dates`            |
| `chart`    | Line/area chart | `datasets`, `x_axis`, `y_axis`             |

#### Feedback Widgets

| Component  | Description        | Properties                            |
| ---------- | ------------------ | ------------------------------------- |
| `spinner`  | Loading spinner    | `style`, `label`, `active`            |
| `throbber` | Animated throbber  | `set`, `label`, `state`               |
| `progress` | Progress indicator | `value`, `max`, `type`                |
| `toast`    | Toast notification | `message`, `type`, `duration`         |
| `alert`    | Alert box          | `type`, `title`, `message`, `buttons` |
| `badge`    | Status badge       | `text`, `color`, `variant`            |

#### Navigation Widgets

| Component     | Description      | Properties                     |
| ------------- | ---------------- | ------------------------------ |
| `menu`        | Dropdown menu    | `items`, `selected`, `nested`  |
| `menubar`     | Top menu bar     | `menus`, `active`              |
| `contextmenu` | Right-click menu | `items`, `x`, `y`              |
| `breadcrumb`  | Path breadcrumb  | `path`, `separator`            |
| `pagination`  | Page navigation  | `total`, `current`, `per_page` |

#### Media Widgets

| Component  | Description      | Properties                                     |
| ---------- | ---------------- | ---------------------------------------------- |
| `image`    | Terminal image   | `src`, `width`, `height`, `protocol`           |
| `video`    | Video player     | `src`, `controls`, `autoplay`                  |
| `audio`    | Audio player     | `src`, `controls`, `volume`                    |
| `markdown` | Markdown render  | `content`, `theme`                             |
| `code`     | Syntax highlight | `content`, `language`, `theme`, `line_numbers` |

---

## 📝 GUL Widget Syntax Examples

### Basic Widgets

```gul
# Button with click handler
^&^[button{text: "Submit", color: "primary", onClick: handle_submit}]

# Text input with placeholder
^&^[input{placeholder: "Enter your name", value: @name, maxLength: 50}]

# Checkbox
^&^[checkbox{label: "I agree to terms", checked: @agreed}]

# Slider with range
^&^[slider{min: 0, max: 100, value: @volume, step: 5}]

# Dropdown select
^&^[select{
    options: ["Option A", "Option B", "Option C"],
    value: @selected,
    placeholder: "Choose one..."
}]
```

### Display Widgets and Objects

```gul
# Large ASCII text banner (tui-big-text inspired)
^&^[bigtext{text: "GUL", font: "standard", color: "cyan"}]

# Progress gauge
^&^[gauge{value: 75, max: 100, label: "Loading...", color: "green"}]

# Sparkline mini-chart
^&^[sparkline{data: [1, 3, 7, 2, 5, 8, 4], color: "yellow"}]

# Bar chart
^&^[barchart{
    data: [10, 25, 15, 30, 20],
    labels: ["Mon", "Tue", "Wed", "Thu", "Fri"],
    color: "blue"
}]
```

### Container Layouts

```gul
# Horizontal row layout
^&^[row{gap: 2, align: "center"}[
    button{text: "Cancel"},
    button{text: "OK", color: "primary"}
]]

# Grid layout
^&^[grid{rows: 2, cols: 3, gap: 1}[
    label{text: "A"}, label{text: "B"}, label{text: "C"},
    label{text: "D"}, label{text: "E"}, label{text: "F"}
]]

# Split panes
^&^[split{direction: "horizontal", ratio: 0.3, resizable: true}[
    tree{nodes: @file_tree},
    code{content: @editor_buffer, language: "gul"}
]]

# Popup modal (tui-popup inspired)
^&^[popup{
    title: "Confirm Delete",
    visible: @show_confirm,
    buttons: ["Cancel", "Delete"]
}[
    paragraph{content: "Are you sure you want to delete this file?"}
]]
```

### Loading Indicators (throbber-widgets-tui inspired)

```gul
# Simple spinner
^&^[spinner{style: "dots", label: "Loading..."}]

# Configurable throbber
^&^[throbber{
    set: "braille",      # braille, ascii, clock, line, block
    label: "Processing",
    state: @loading_state
}]

# Throbber sets available:
# - braille: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
# - ascii: - \ | /
# - clock: 🕐 🕑 🕒 🕓 🕔 🕕 🕖 🕗 🕘 🕙 🕚 🕛
# - line: ▁ ▂ ▃ ▄ ▅ ▆ ▇ █
# - block: ░ ▒ ▓ █
```

### Data Display

```gul
# Data table with selection
^&^[table{
    headers: ["Name", "Age", "City"],
    data: @users,
    selectable: true,
    striped: true,
    selected: @selected_row
}]

# Tree view for file browser
^&^[tree{
    nodes: [
        {name: "src", children: [
            {name: "main.mn"},
            {name: "lib.def"},
            {name: "utils.fnc"}
        ]},
        {name: "tests", children: [
            {name: "test_main.mn"}
        ]}
    ],
    expanded: @expanded_nodes,
    selectable: true
}]

# List with multi-select
^&^[list{
    items: @todo_items,
    selected: @selected_items,
    multi_select: true
}]
```

### Menu System (tui-menu inspired)

```gul
# Menu bar
^&^[menubar{
    menus: [
        {label: "File", items: [
            {label: "New", shortcut: "Ctrl+N"},
            {label: "Open", shortcut: "Ctrl+O"},
            {separator: true},
            {label: "Exit", shortcut: "Ctrl+Q"}
        ]},
        {label: "Edit", items: [
            {label: "Undo", shortcut: "Ctrl+Z"},
            {label: "Redo", shortcut: "Ctrl+Shift+Z"}
        ]},
        {label: "View", items: [
            {label: "Zoom In", shortcut: "Ctrl++"},
            {label: "Zoom Out", shortcut: "Ctrl+-"}
        ]}
    ]
}]

# Context menu (right-click)
^&^[contextmenu{
    items: [
        {label: "Cut", icon: "✂️"},
        {label: "Copy", icon: "📋"},
        {label: "Paste", icon: "📌"},
        {separator: true},
        {label: "Delete", icon: "🗑️", danger: true}
    ],
    x: @menu_x,
    y: @menu_y
}]
```

### Image Rendering (ratatui-image inspired)

```gul
# Render image in terminal
^&^[image{
    src: "logo.png",
    width: 40,
    height: 20,
    protocol: "auto"  # auto, sixel, kitty, iterm2, halfblocks
}]

# Responsive image
^&^[image{
    src: @image_path,
    fit: "contain",     # contain, cover, fill
    alt: "Preview"
}]
```

### Scrollable Content (tui-scrollview inspired)

```gul
# Scrollable text view
^&^[scrollview{
    scroll_x: true,
    scroll_y: true,
    content: paragraph{content: @long_text}
}]

# Scrollable with viewport
^&^[scrollview{
    viewport_height: 20,
    viewport_width: 80,
    content: ^&^[code{content: @source_code, language: "gul"}]
}]
```

### Prompt Dialogs (tui-prompts inspired)

```gul
# Input prompt
^&^[prompt{
    type: "input",
    question: "Enter project name:",
    default: "my-project",
    onSubmit: handle_name_input
}]

# Confirmation prompt
^&^[prompt{
    type: "confirm",
    question: "Delete this file?",
    default: false,
    onConfirm: handle_confirm
}]

# Selection prompt
^&^[prompt{
    type: "select",
    question: "Choose template:",
    options: ["Basic", "Web App", "API", "CLI"],
    onSelect: handle_template
}]
```

---

## 🎯 Current State Analysis

### Current TUI Implementation (`src/tools/tui_ide.rs`)

**Strengths:**

- Basic structure exists
- File browser component
- Command palette
- File operations
- v2.1 syntax highlighting ✅
- Bracket equivalence support ✅

**Weaknesses:**

- No actual Ratatui integration (placeholder only)
- No terminal rendering
- No event loop
- Limited UI widgets

**Current Dependencies:**

- `crossterm = "0.27"` - Terminal handling
- `unicode-width = "0.1"` - Character width
- Missing: `ratatui`, `syntect`, `tree-sitter`, widget extensions

---

## 🏗️ Upgrade Architecture

### Phase 1: Core Framework (Week 1-2)

#### 1.1 Add Dependencies

```toml
[dependencies]
# TUI Framework
ratatui = { version = "0.29", features = ["all-widgets"] }
crossterm = "0.28"

# Widget Extensions
tui-widgets = "0.3"           # Big text, popup, prompts, scrollview
ratatui-image = "4.0"         # Image rendering
throbber-widgets-tui = "0.8"  # Spinners and throbbers
tui-menu = "0.5"              # Menu system

# Syntax Highlighting
syntect = "5.2"
tree-sitter = "0.24"
tree-sitter-gul = { path = "crates/tree-sitter-gul" }

# Async Event Loop
tokio = { version = "1.35", features = ["full", "sync"] }

# Optional: Web TUI
ratzilla = { version = "0.5", optional = true }
```

#### 1.2 Create Tree-Sitter Grammar for GUL/MN

```text
crates/
└── tree-sitter-gul/
    ├── Cargo.toml
    ├── grammar.js
    ├── src/
    │   ├── grammar.json
    │   ├── parser.c
    │   └── scanner.c
    └── queries/
        ├── highlights.scm
        └── injections.scm
```

**Highlights query for v2.1:**

```scheme
; highlights.scm - GUL v2.1 Bracket Equivalence

; Keywords
["main" "fn" "async" "struct" "import" "const" "mut"] @keyword

; All bracket types equivalent
["(" "[" "{"] @punctuation.bracket
[")" "]" "}"] @punctuation.bracket

; UI component syntax
"^&^" @ui.marker
(ui_component) @ui.component

; File types
(file_extension) @type.builtin
".mn" @type.main
".def" @type.definition
".fnc" @type.function
".cs" @type.crossscript
".sct" @type.secret_credential
```

### Phase 2: Widget System (Week 3-4)

#### 2.1 Core Widgets

```rust
// src/tui/widgets/mod.rs

pub mod editor;        // Code editor with syntax highlighting
pub mod file_tree;     // Project file browser
pub mod terminal;      // Integrated terminal
pub mod output;        // Compiler output panel
pub mod minimap;       // Code minimap
pub mod tabs;          // Tab bar for open files
pub mod status_bar;    // Status bar with git, cursor pos
pub mod command_palette; // Quick command search
pub mod diagnostics;   // Error/warning panel
pub mod image;         // Terminal image rendering
pub mod throbber;      // Loading spinners
pub mod menu;          // Menu system
pub mod popup;         // Modal popups
pub mod scrollview;    // Scrollable containers
pub mod bigtext;       // ASCII art text
```

#### 2.2 Editor Widget with Bracket Highlighting

```rust
pub struct EditorWidget {
    buffer: rope_data_structure::Rope,
    syntax: SyntaxHighlighter,
    cursor: Position,
    selections: Vec<Selection>,
    bracket_pairs: Vec<BracketPair>,
}

impl EditorWidget {
    /// Highlight matching brackets (v2.1 equivalence)
    pub fn highlight_bracket_pair(&self, pos: Position) -> Option<(Position, Position)> {
        // Find opening bracket at position
        let char_at = self.char_at(pos);

        // v2.1: (, [, { are all interchangeable
        if is_open_bracket(char_at) {
            let close = self.find_matching_close(pos);
            // Highlight both even if different types (v2.1)
            return Some((pos, close));
        }

        if is_close_bracket(char_at) {
            let open = self.find_matching_open(pos);
            return Some((open, pos));
        }

        None
    }
}
```

### Phase 3: Syntax Highlighting (Week 5-6)

#### 3.1 GUL/MN Syntax Theme

```rust
// src/tui/syntax/gul_theme.rs

pub struct GulTheme {
    pub keyword: Style,
    pub function: Style,
    pub string: Style,
    pub number: Style,
    pub comment: Style,
    pub ui_marker: Style,       // ^&^ highlighting
    pub ui_component: Style,    // button, table, etc.
    pub bracket_match: Style,   // v2.1 bracket pairs
    pub file_type: Style,       // .mn, .def, .fnc, .cs, .sct
}

impl Default for GulTheme {
    fn default() -> Self {
        GulTheme {
            keyword: Style::default().fg(Color::Magenta).bold(),
            function: Style::default().fg(Color::Blue),
            string: Style::default().fg(Color::Green),
            number: Style::default().fg(Color::Yellow),
            comment: Style::default().fg(Color::DarkGray).italic(),
            ui_marker: Style::default().fg(Color::Cyan).bold(),
            ui_component: Style::default().fg(Color::LightCyan),
            bracket_match: Style::default().bg(Color::DarkGray),
            file_type: Style::default().fg(Color::LightBlue),
        }
    }
}
```

#### 3.2 Token Classification for v2.1

```rust
pub enum TokenType {
    // Core
    Keyword,
    Identifier,
    Literal,

    // v2.1 Brackets (all equivalent semantically)
    BracketOpen,   // ( [ {
    BracketClose,  // ) ] }

    // v2.1 UI Syntax
    UiMarker,      // ^&^
    UiComponent,   // button, table, spinner, etc.
    UiProperty,    // text:, data:, etc.

    // v2.1 File Types
    FileTypeMain,  // .mn
    FileTypeDef,   // .def
    FileTypeFnc,   // .fnc
    FileTypeCs,    // .cs
    FileTypeSct,   // .sct
}
```

### Phase 4: Application Shell (Week 7-8)

#### 4.1 Main Application

```rust
// src/tui/app.rs

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::*,
    layout::*,
};
use throbber_widgets_tui::Throbber;
use ratatui_image::StatefulImage;
use tui_widgets::{popup::Popup, scrollview::ScrollView};
use tui_menu::Menu;

pub struct GulIdeApp {
    // Layout
    layout: AppLayout,

    // Widgets
    editor: EditorWidget,
    file_tree: FileTreeWidget,
    terminal: TerminalWidget,
    output: OutputWidget,
    tabs: TabsWidget,
    status: StatusBarWidget,
    diagnostics: DiagnosticsWidget,

    // Extended widgets
    loading: Option<ThrobberState>,
    preview_image: Option<StatefulProtocol>,
    menu: Option<MenuState>,
    popup: Option<PopupState>,

    // State
    mode: Mode,
    focus: Focus,
    project: Option<Project>,
}

impl GulIdeApp {
    pub fn run(&mut self) -> io::Result<()> {
        // Initialize terminal
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        // Event loop
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                if self.handle_key(key)? {
                    break;
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),  // File tree
                Constraint::Percentage(60),  // Editor
                Constraint::Percentage(20),  // Output
            ])
            .split(frame.area());

        self.file_tree.render(frame, chunks[0]);
        self.editor.render(frame, chunks[1]);
        self.output.render(frame, chunks[2]);

        // Render loading indicator if active
        if let Some(ref mut state) = self.loading {
            let throbber = Throbber::default()
                .label("Building...")
                .throbber_set(throbber_widgets_tui::BRAILLE_SIX);
            frame.render_stateful_widget(throbber, status_area, state);
        }

        // Render popup if visible
        if let Some(ref popup) = self.popup {
            let popup_widget = Popup::new("Confirm", popup.content.clone());
            frame.render_widget(popup_widget, centered_rect(50, 30, frame.area()));
        }
    }
}
```

### Phase 5: Web TUI with Ratzilla (Week 9-10)

#### 5.1 WebAssembly Target

```rust
// src/tui/web/mod.rs

#[cfg(feature = "web-tui")]
pub mod web_app {
    use ratzilla::prelude::*;

    pub fn run_web_ide() {
        ratzilla::run(|term| {
            let mut app = GulIdeApp::new();

            loop {
                term.draw(|frame| app.render(frame))?;

                if let Some(event) = term.poll_event() {
                    if app.handle_event(event)? {
                        break;
                    }
                }
            }

            Ok(())
        });
    }
}
```

#### 5.2 Deployment Configuration

```toml
# ratzilla.toml
[build]
target = "wasm32-unknown-unknown"
out_dir = "dist"

[deploy]
provider = "vercel"
```

---

## 📋 Implementation Checklist

### Phase 1: Core Framework

- [x] Add Ratatui dependency
- [x] Update Crossterm to 0.28
- [ ] Add tui-widgets, ratatui-image, throbber-widgets-tui, tui-menu
- [x] Create tree-sitter-gul grammar
- [x] Create highlights.scm query
- [x] Create injections.scm query
- [x] Create locals.scm query
- [x] Implement v2.1 bracket queries
- [x] Basic terminal initialization

### Phase 2: Widget System

- [x] EditorWidget with buffer
- [x] FileTreeWidget with .mn/.sct support
- [x] TerminalWidget for build output (OutputWidget)
- [x] TabsWidget for multiple files
- [x] StatusBarWidget with git
- [x] CommandPaletteWidget
- [x] ImageWidget (multi-protocol: Sixel, Kitty, iTerm2, Halfblocks)
- [x] ThrobberWidget (10 spinner styles: braille, dots, arrows, etc.)
- [x] MenuWidget (context menu, menu bar)
- [x] PopupWidget (alerts, confirmations, dialogs)
- [x] ScrollViewWidget (vertical/horizontal scrolling)

### Phase 3: Syntax Highlighting

- [x] GUL theme definition (dark & light)
- [x] v2.1 bracket equivalence colors
- [x] UI component highlighting (^&^)
- [x] File type indicators (.mn, .sct, etc.)
- [x] Bracket pair matching

### Phase 4: Application Shell

- [x] Main event loop
- [x] Keyboard handling
- [ ] Mouse support
- [x] Layout management
- [x] Focus navigation
- [x] Menu bar (MenuBarWidget)
- [x] Context menus (ContextMenuWidget)

### Phase 5: Web TUI

- [x] Ratzilla integration (WebTui, WebConfig)
- [x] WASM build configuration (WasmBuildConfig)
- [x] Web deployment (DeployConfig, DeployProvider)
- [ ] Browser testing

### Phase 6: Polish

- [x] Animations (Harmonica-inspired: Spring, Tween, Easing)
- [x] Code screenshots (Freeze-inspired: ScreenshotBuilder, SVG/HTML/ANSI)
- [x] Markdown preview (Glow-inspired)
- [x] Image preview (ImageWidget)
- [x] Fuzzy finder (television-inspired)
- [x] Dashboard widgets (SparklineWidget, GaugeWidget, BarChartWidget, DonutWidget)
- [ ] Performance optimization

---

## 🎨 Design Inspiration

### From Charmbracelet Stack

| Feature           | Inspiration | GUL Implementation             |
| ----------------- | ----------- | ------------------------------ |
| Elm Architecture  | Bubbletea   | Component message passing      |
| Spring Animations | Harmonica   | Smooth scroll, cursor movement |
| Markdown Render   | Glow        | README preview, doc comments   |
| Code Screenshots  | Freeze      | Export code as PNG             |
| Git Integration   | Soft-Serve  | Commit, diff, branch display   |
| High Performance  | Ultraviolet | Optimized rendering            |

### From Widget Libraries

| Feature          | Library              | GUL Implementation               |
| ---------------- | -------------------- | -------------------------------- |
| Big Text Banners | tui-big-text         | `^&^[bigtext{text: "GUL"}]`      |
| Modal Popups     | tui-popup            | `^&^[popup{title: "..."}]`       |
| Input Prompts    | tui-prompts          | `^&^[prompt{type: "input"}]`     |
| Scrollable Areas | tui-scrollview       | `^&^[scrollview{...}]`           |
| Terminal Images  | ratatui-image        | `^&^[image{src: "..."}]`         |
| Loading Spinners | throbber-widgets-tui | `^&^[spinner{style: "braille"}]` |
| Nested Menus     | tui-menu             | `^&^[menu{items: [...]}]`        |

### UI Mockup

```text
┌─────────────────────────────────────────────────────────────────┐
│ File  Edit  View  Help                           ▪ ▫ ✕          │
├──────────────┬──────────────────────────────────────────────────┤
│ 📁 Project   │  1 │ # GUL v2.1 Example                          │
│  ├ src/      │  2 │                                              │
│  │ ├ main.mn │  3 │ import std{io, http}                        │
│  │ ├ lib.def │  4 │                                              │
│  │ ├ api.fnc │  5 │ main:                                        │
│  │ └ 🔐.sct  │  6 │     # Loading indicator                      │
│  ├ tests/    │  7 │     ^&^[spinner{style: "braille"}]           │
│  └ README.md │  8 │                                              │
├──────────────┤  9 │     # Data table                             │
│ 📊 Output    │ 10 │     ^&^[table{data: users, headers: [...]}]  │
│              │ 11 │                                              │
│ ⠙ Building   │ 12 │     # Menu bar                               │
│              │ 13 │     ^&^[menubar{menus: [...]}]               │
├──────────────┴──────────────────────────────────────────────────┤
│ Ln 6, Col 5   │ main.mn │ UTF-8 │ GUL │ ⚡ main │ 🔀 feat/v2.1  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📦 Deliverables

1. **Core TUI IDE** (`src/tui/`) - Full Ratatui implementation
2. **Tree-Sitter Grammar** (`crates/tree-sitter-gul/`) - GUL v2.1 parsing
3. **Syntax Themes** (`themes/`) - Light/dark themes
4. **Widget Library** (`src/tui/widgets/`) - All native ^&^ components
5. **Web TUI** (`src/tui/web/`) - Ratzilla WebAssembly build
6. **Documentation** (`docs/guides/tui-ide.md`) - User guide
7. **UI Component Reference** (`docs/reference/ui-components.md`) - Widget docs

## 🔗 References

### Core Frameworks

- [Ratatui Documentation](https://docs.rs/ratatui)
- [Ratatui Examples](https://github.com/ratatui/ratatui/tree/main/examples)
- [Ratzilla - Web TUI](https://github.com/orhun/ratzilla)
- [egui_ratatui](https://github.com/gold-silver-copper/egui_ratatui)

### Widget Extensions (inspired by)

- [tui-widgets](https://github.com/joshka/tui-widgets) - Big text, popup, prompts, scrollview
- [ratatui-image](https://github.com/benjajaja/ratatui-image) - Terminal image rendering
- [throbber-widgets-tui](https://github.com/arkbig/throbber-widgets-tui) - Loading spinners
- [tui-menu](https://github.com/shuoli84/tui-menu) - Nested menu system

### Dashboard & Fuzzy Finder Inspiration

- [television](https://github.com/alexpasmantier/television) - Portable fuzzy finder with channels
- [termui](https://github.com/gizak/termui) - Go terminal dashboard widgets
- [blessed-contrib](https://github.com/yaronn/blessed-contrib) - Node.js dashboard widgets
- [blessed](https://github.com/chjj/blessed) - Comprehensive Node.js TUI library

### Charmbracelet Stack

- [Bubbletea Tutorial](https://github.com/charmbracelet/bubbletea/tree/main/tutorials)
- [Bubbles Components](https://github.com/charmbracelet/bubbles)
- [Glow - Markdown Renderer](https://github.com/charmbracelet/glow)
- [Freeze - Code Screenshots](https://github.com/charmbracelet/freeze)
- [Harmonica - Animations](https://github.com/charmbracelet/harmonica)

### Parsing & Syntax

- [Tree-Sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Tree-Sitter Playground](https://tree-sitter.github.io/tree-sitter/playground)

---

**Document Version**: 3.0.0  
**Last Updated**: 2025-12-11  
**Author**: GUL Development Team
