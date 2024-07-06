use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::canvas::Context;

use super::debug_lines::DebugLines;
use super::on_event::OnEvent;
use crate::app::ForbiddenPoints;

pub trait Object: Debug + OnEvent + DebugLines {
    #[allow(unused_variables)]
    fn render(&mut self, area: Rect, buf: &mut Buffer, forbidden_points: Rc<RefCell<ForbiddenPoints>>) {}
    #[allow(unused_variables)]
    fn draw(&mut self, ctx: &mut Context, forbidden_points: Rc<RefCell<ForbiddenPoints>>) {}
}
