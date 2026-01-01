// GUL TUI Module - Terminal User Interface
// Complete implementation with Ratatui

pub mod animation;
pub mod app;
pub mod events;
pub mod screenshot;
pub mod theme;
pub mod web;
pub mod widgets;

pub use animation::{
    CursorAnimation, Easing, FadeAnimation, ScrollAnimation, Spring, SpringConfig, Tween,
};
pub use app::GulTuiApp;
pub use screenshot::{
    ScreenshotBuilder, ScreenshotConfig, ScreenshotFormat, ScreenshotTheme, WindowDecoration,
};
pub use theme::GulTheme;
pub use web::{DeployConfig, DeployProvider, WasmBuildConfig};
