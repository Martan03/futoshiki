use std::{cell::RefCell, rc::Rc};

use futoshiki_core::board::board_struct::Board;
use termint::{
    buffer::Buffer,
    enums::Modifier,
    geometry::{Rect, Vec2},
    style::Style,
    widgets::{Element, LayoutNode, Widget},
};

use crate::tui::theme::Theme;

#[derive(Debug)]
pub struct BoardTui {
    board: Rc<RefCell<Board>>,
    theme: Theme,
}

impl BoardTui {
    pub fn new(board: Rc<RefCell<Board>>, theme: Theme) -> Self {
        Self { board, theme }
    }
}

impl Widget for BoardTui {
    fn render(&self, buffer: &mut Buffer, layout: &LayoutNode) {
        self.render_inner(buffer, &layout.area);
        self.render_outer(buffer, &layout.area);
        self.render_cells(buffer, &layout.area);
        self.render_sel(buffer, &layout.area);
        self.render_ver_conds(buffer, &layout.area);
        self.render_hor_conds(buffer, &layout.area);
    }

    fn height(&self, _size: &Vec2) -> usize {
        self.board.borrow().size() * 2 + 1
    }

    fn width(&self, _size: &Vec2) -> usize {
        self.board.borrow().size() * 4 + 1
    }
}

impl BoardTui {
    /// Renders selected border
    fn render_sel(&self, buffer: &mut Buffer, rect: &Rect) {
        let board = self.board.borrow();
        let sel_x = rect.x() + board.selected.x * 4;
        let sel_y = rect.y() + board.selected.y * 2;

        let sel = &board.selected;
        let size = board.size();

        let (top, bottom) = match (sel.x, sel.y) {
            (0, 0) => ("┏━━━┱", "┡━━━╃"),
            (0, y) if y + 1 == size => ("┢━━━╅", "┗━━━┹"),
            (x, 0) if x + 1 == size => ("┲━━━┓", "╄━━━┩"),
            (x, y) if x + 1 == size && y + 1 == size => ("╆━━━┪", "┺━━━┛"),
            (_, 0) => ("┲━━━┱", "╄━━━╃"),
            (_, y) if y + 1 == size => ("╆━━━╅", "┺━━━┹"),
            (0, _) => ("┢━━━╅", "┡━━━╃"),
            (x, _) if x + 1 == size => ("╆━━━┪", "╄━━━┩"),
            _ => ("╆━━━╅", "╄━━━╃"),
        };
        buffer.set_str(top, &Vec2::new(sel_x, sel_y));
        buffer.set_str(bottom, &Vec2::new(sel_x, sel_y + 2));
        buffer.set_val("┃", &Vec2::new(sel_x, sel_y + 1));
        buffer.set_val("┃", &Vec2::new(sel_x + 4, sel_y + 1));
    }

    /// Renders cells
    fn render_cells(&self, buffer: &mut Buffer, rect: &Rect) {
        let board = self.board.borrow();
        let mut coords = Vec2::new(rect.x() + 2, rect.y() + 1);
        let size = board.size();
        let mut id = 0;
        for _ in 0..size {
            for _ in 0..size {
                let cell = board[id];
                match cell.value() {
                    0 => {}
                    val if cell.enabled() => buffer.set_str_styled(
                        val.to_string(),
                        &coords,
                        Style::new().fg(self.theme.foreground),
                    ),
                    val => {
                        buffer.set_str_styled(
                            val.to_string(),
                            &coords,
                            Style::new()
                                .fg(self.theme.foreground)
                                .modifier(Modifier::BOLD),
                        );
                    }
                }
                id += 1;
                coords.x += 4;
            }
            coords.y += 2;
            coords.x = rect.x() + 2;
        }
    }

    /// Renders outer borders
    fn render_outer(&self, buffer: &mut Buffer, rect: &Rect) {
        let board = self.board.borrow();
        let bottom = board.size() * 2;
        let right = board.size() * 4;

        buffer.set_str_styled(
            "───┬".repeat(board.size()),
            &Vec2::new(rect.x() + 1, rect.y()),
            Style::new().fg(self.theme.border),
        );
        buffer.set_str_styled(
            "───┴".repeat(board.size()),
            &Vec2::new(rect.x() + 1, rect.y() + bottom),
            Style::new().fg(self.theme.border),
        );

        let mut leftc = Vec2::new(rect.x(), rect.y() + 1);
        let mut rightc = Vec2::new(rect.x() + right, rect.y() + 1);
        for _ in rect.y()..rect.y() + board.size() {
            self.border_part("│", buffer, &leftc);
            leftc.y += 1;
            self.border_part("├", buffer, &leftc);
            leftc.y += 1;

            self.border_part("│", buffer, &rightc);
            rightc.y += 1;
            self.border_part("┤", buffer, &rightc);
            rightc.y += 1;
        }

        let mut pos = *rect.pos();
        self.border_part("┌", buffer, &pos);
        pos.x += right;
        self.border_part("┐", buffer, &pos);
        pos.y += bottom;
        self.border_part("┘", buffer, &pos);
        pos.x -= right;
        self.border_part("└", buffer, &pos);
    }

    /// Renders inner borders
    fn render_inner(&self, buffer: &mut Buffer, rect: &Rect) {
        let board = self.board.borrow();
        let line = "───┼".repeat(board.size());
        for y in 1..board.size() {
            buffer.set_str_styled(
                &line,
                &Vec2::new(rect.x() + 1, rect.y() + y * 2),
                Style::new().fg(self.theme.border),
            );
        }

        let line = "   │".repeat(board.size());
        for y in 0..board.size() {
            buffer.set_str_styled(
                &line,
                &Vec2::new(rect.x() + 1, rect.y() + y * 2 + 1),
                Style::new().fg(self.theme.border),
            )
        }
    }

    fn render_hor_conds(&self, buffer: &mut Buffer, rect: &Rect) {
        let board = self.board.borrow();
        let size = Vec2::new(board.size().saturating_sub(1), board.size());
        for y in 0..size.y {
            for x in 0..size.x {
                let c = match board.hor_conds[x + y * size.x] {
                    Some(true) => ">",
                    Some(false) => "<",
                    _ => continue,
                };
                let pos =
                    Vec2::new(rect.x() + x * 4 + 4, rect.y() + y * 2 + 1);
                buffer.set_val(c, &pos);
                buffer.set_fg(self.theme.select, &pos);
            }
        }
    }

    fn render_ver_conds(&self, buffer: &mut Buffer, rect: &Rect) {
        let board = self.board.borrow();
        let size = Vec2::new(board.size(), board.size().saturating_sub(1));
        for y in 0..size.y {
            for x in 0..size.x {
                let c = match board.ver_conds[x + y * size.x] {
                    Some(true) => "∨",
                    Some(false) => "∧",
                    _ => continue,
                };
                let pos =
                    Vec2::new(rect.x() + x * 4 + 2, rect.y() + y * 2 + 2);
                buffer.set_val(c, &pos);
                buffer.set_fg(self.theme.select, &pos);
            }
        }
    }

    /// Renders part of the border
    fn border_part(&self, val: &str, buffer: &mut Buffer, pos: &Vec2) {
        buffer.set_val(val, pos);
        buffer.set_fg(self.theme.border, pos);
    }
}

impl From<BoardTui> for Element {
    fn from(value: BoardTui) -> Self {
        Element::new(value)
    }
}

impl From<BoardTui> for Box<dyn Widget> {
    fn from(value: BoardTui) -> Self {
        Box::new(value)
    }
}
