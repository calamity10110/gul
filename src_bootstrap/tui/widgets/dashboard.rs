// GUL TUI Dashboard Widgets
// Charts, gauges, and data visualization
// Inspired by termui and blessed-contrib

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

use crate::tui::theme::GulTheme;

/// Sparkline data
pub struct SparklineWidget<'a> {
    /// Data points
    data: &'a [u64],
    /// Color
    color: Color,
    /// Max value (auto if None)
    max: Option<u64>,
    /// Title
    title: Option<&'a str>,
    /// Theme
    #[allow(dead_code)]
    theme: &'a GulTheme,
}

impl<'a> SparklineWidget<'a> {
    pub fn new(data: &'a [u64], theme: &'a GulTheme) -> Self {
        SparklineWidget {
            data,
            color: Color::Green,
            max: None,
            title: None,
            theme,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

impl<'a> Widget for SparklineWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.data.is_empty() || area.width == 0 || area.height == 0 {
            return;
        }

        let max = self
            .max
            .unwrap_or_else(|| *self.data.iter().max().unwrap_or(&1));
        let blocks = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

        let style = Style::default().fg(self.color);
        let data_len = self.data.len().min(area.width as usize);

        for (i, &value) in self.data.iter().take(data_len).enumerate() {
            let x = area.x + i as u16;
            let height_ratio = if max > 0 {
                (value as f64 / max as f64).min(1.0)
            } else {
                0.0
            };
            let block_idx = (height_ratio * 7.0) as usize;
            buf.set_string(x, area.y, blocks[block_idx], style);
        }

        // Title
        if let Some(title) = self.title {
            if area.height > 1 {
                buf.set_string(area.x, area.y + 1, title, Style::default());
            }
        }
    }
}

/// Gauge widget (progress bar)
pub struct GaugeWidget<'a> {
    /// Progress value (0.0 - 1.0)
    ratio: f64,
    /// Label
    label: Option<&'a str>,
    /// Color
    color: Color,
    /// Background color
    bg_color: Color,
    /// Show percentage
    show_percent: bool,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> GaugeWidget<'a> {
    pub fn new(ratio: f64, theme: &'a GulTheme) -> Self {
        GaugeWidget {
            ratio: ratio.clamp(0.0, 1.0),
            label: None,
            color: Color::Green,
            bg_color: Color::DarkGray,
            show_percent: true,
            theme,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }
}

impl<'a> Widget for GaugeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        // Draw background
        let bg_style = Style::default().bg(self.bg_color);
        for x in inner.x..inner.x + inner.width {
            buf.set_string(x, inner.y, " ", bg_style);
        }

        // Draw filled portion
        let filled_width = (inner.width as f64 * self.ratio) as u16;
        let fg_style = Style::default().bg(self.color);
        for x in inner.x..inner.x + filled_width {
            buf.set_string(x, inner.y, " ", fg_style);
        }

        // Draw percentage or label
        let text = if let Some(label) = self.label {
            label.to_string()
        } else if self.show_percent {
            format!("{}%", (self.ratio * 100.0) as u8)
        } else {
            String::new()
        };

        if !text.is_empty() {
            let text_x = inner.x + (inner.width.saturating_sub(text.len() as u16)) / 2;
            buf.set_string(text_x, inner.y, &text, Style::default());
        }
    }
}

/// Simple bar chart
pub struct BarChartWidget<'a> {
    /// Data points
    data: &'a [(String, u64)],
    /// Bar color
    bar_color: Color,
    /// Max value
    max: Option<u64>,
    /// Title
    title: Option<&'a str>,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> BarChartWidget<'a> {
    pub fn new(data: &'a [(String, u64)], theme: &'a GulTheme) -> Self {
        BarChartWidget {
            data,
            bar_color: Color::Blue,
            max: None,
            title: None,
            theme,
        }
    }

    pub fn bar_color(mut self, color: Color) -> Self {
        self.bar_color = color;
        self
    }

    pub fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

impl<'a> Widget for BarChartWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = if let Some(title) = self.title {
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        };

        let inner = block.inner(area);
        block.render(area, buf);

        if self.data.is_empty() || inner.width == 0 || inner.height == 0 {
            return;
        }

        let max = self
            .max
            .unwrap_or_else(|| self.data.iter().map(|(_, v)| *v).max().unwrap_or(1));
        let bar_width = (inner.width as usize / self.data.len()).max(1) as u16;
        let chart_height = inner.height.saturating_sub(1); // Leave room for labels

        let bar_style = Style::default().fg(self.bar_color);
        let _blocks = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

        for (i, (label, value)) in self.data.iter().enumerate() {
            let x_start = inner.x + (i as u16 * bar_width);

            // Draw bar
            let height_ratio = if max > 0 {
                (*value as f64 / max as f64).min(1.0)
            } else {
                0.0
            };
            let bar_height = (chart_height as f64 * height_ratio) as u16;

            for y in 0..bar_height {
                let y_pos = inner.y + chart_height - 1 - y;
                buf.set_string(x_start, y_pos, "█", bar_style);
            }

            // Draw label (truncated)
            let label_y = inner.y + inner.height - 1;
            let truncated_label: String = label.chars().take(bar_width as usize).collect();
            buf.set_string(x_start, label_y, &truncated_label, Style::default());
        }
    }
}

/// Donut chart segment
#[derive(Debug, Clone)]
pub struct DonutSegment {
    pub label: String,
    pub value: f64,
    pub color: Color,
}

/// Donut chart widget
pub struct DonutWidget<'a> {
    segments: &'a [DonutSegment],
    theme: &'a GulTheme,
    title: Option<&'a str>,
}

impl<'a> DonutWidget<'a> {
    pub fn new(segments: &'a [DonutSegment], theme: &'a GulTheme) -> Self {
        DonutWidget {
            segments,
            theme,
            title: None,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

impl<'a> Widget for DonutWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = if let Some(title) = self.title {
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        };

        let inner = block.inner(area);
        block.render(area, buf);

        if self.segments.is_empty() || inner.width < 10 || inner.height < 5 {
            // Fallback to simple text representation
            let total: f64 = self.segments.iter().map(|s| s.value).sum();
            for (i, segment) in self.segments.iter().enumerate() {
                if i as u16 >= inner.height {
                    break;
                }
                let percent = if total > 0.0 {
                    segment.value / total * 100.0
                } else {
                    0.0
                };
                let text = format!("● {}: {:.1}%", segment.label, percent);
                buf.set_string(
                    inner.x,
                    inner.y + i as u16,
                    &text,
                    Style::default().fg(segment.color),
                );
            }
            return;
        }

        // Simple ASCII donut representation
        let _radius = (inner.height / 2).min(inner.width / 4) as i32;
        let cx = (inner.x + inner.width / 2) as i32;
        let cy = (inner.y + inner.height / 2) as i32;

        // Draw a simple circle using braille-like characters
        let _circle_chars = ['○', '◔', '◑', '◕', '●'];
        let total: f64 = self.segments.iter().map(|s| s.value).sum();

        if total > 0.0 && !self.segments.is_empty() {
            // Just show the first segment's percentage in the center
            let first_percent = self.segments[0].value / total * 100.0;
            let text = format!("{:.0}%", first_percent);
            let text_x = (cx - (text.len() as i32 / 2)) as u16;
            buf.set_string(
                text_x,
                cy as u16,
                &text,
                Style::default().fg(self.segments[0].color),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline() {
        let theme = GulTheme::default();
        let data = vec![1, 3, 7, 2, 5, 8, 4];
        let sparkline = SparklineWidget::new(&data, &theme)
            .color(Color::Yellow)
            .title("Test");

        assert!(sparkline.max.is_none());
    }

    #[test]
    fn test_gauge() {
        let theme = GulTheme::default();
        let gauge = GaugeWidget::new(0.75, &theme)
            .label("Progress")
            .color(Color::Green);

        assert_eq!(gauge.ratio, 0.75);
    }

    #[test]
    fn test_barchart() {
        let theme = GulTheme::default();
        let data = vec![
            ("Mon".to_string(), 10),
            ("Tue".to_string(), 25),
            ("Wed".to_string(), 15),
        ];

        let chart = BarChartWidget::new(&data, &theme)
            .bar_color(Color::Blue)
            .title("Weekly");

        assert!(chart.max.is_none());
    }
}
