use ratatui::text::Line;

pub trait DebugLines {
    fn debug_lines(&self) -> Vec<Line> { vec![] }
}
