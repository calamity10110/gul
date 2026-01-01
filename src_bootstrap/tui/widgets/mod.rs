// GUL TUI Widgets Module
// Complete widget library for the TUI IDE

pub mod command_palette;
pub mod dashboard;
pub mod editor;
pub mod file_tree;
pub mod fuzzy_finder;
pub mod image;
pub mod markdown;
pub mod menu;
pub mod output;
pub mod popup;
pub mod scrollview;
pub mod status_bar;
pub mod tabs;
pub mod throbber;

// Re-exports
pub use command_palette::{Command, CommandPaletteState, CommandPaletteWidget};
pub use dashboard::{BarChartWidget, DonutSegment, DonutWidget, GaugeWidget, SparklineWidget};
pub use editor::{EditorState, EditorWidget};
pub use file_tree::{FileEntry, FileTreeState, FileTreeWidget};
pub use fuzzy_finder::{
    fuzzy_match, CommandFinder, FileFinder, FuzzyFinderState, FuzzyFinderWidget, FuzzyMatch,
};
pub use image::{ImageFit, ImageProtocol, ImageState, ImageWidget};
pub use markdown::{parse_markdown, MarkdownElement, MarkdownState, MarkdownWidget};
pub use menu::{ContextMenuWidget, MenuBarWidget, MenuItem, MenuState, MenuWidget};
pub use output::{OutputLine, OutputType, OutputWidget};
pub use popup::{AlertWidget, ConfirmWidget, PopupButton, PopupState, PopupType, PopupWidget};
pub use scrollview::{ScrollDirection, ScrollViewState, ScrollViewWidget, ScrollableText};
pub use status_bar::StatusBarWidget;
pub use tabs::{Tab, TabsWidget};
pub use throbber::{SpinnerStyle, SpinnerWidget, ThrobberSet, ThrobberState, ThrobberWidget};
