use termint::{
    buffer::Buffer,
    enums::{Color, Modifier},
    geometry::Vec2,
    style::Style,
    widgets::Widget,
};

use super::board_struct::Board;

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        self.render_inner(buffer);
        self.render_outer(buffer);
        self.render_cells(buffer);
        self.render_sel(buffer);
        self.render_ver_conds(buffer);
        self.render_hor_conds(buffer);
    }

    fn height(&self, _size: &Vec2) -> usize {
        self.size() * 2 + 1
    }

    fn width(&self, _size: &Vec2) -> usize {
        self.size() * 4 + 1
    }
}

impl Board {
    /// Renders selected border
    fn render_sel(&self, buffer: &mut Buffer) {
        let sel_x = buffer.x() + self.selected.x * 4;
        let sel_y = buffer.y() + self.selected.y * 2;

        let (top, bottom) = match (self.selected.x, self.selected.y) {
            (0, 0) => ("┏━━━┱", "┡━━━╃"),
            (0, y) if y + 1 == self.size() => ("┢━━━╅", "┗━━━┹"),
            (x, 0) if x + 1 == self.size() => ("┲━━━┓", "╄━━━┩"),
            (x, y) if x + 1 == self.size() && y + 1 == self.size() => {
                ("╆━━━┪", "┺━━━┛")
            }
            (_, 0) => ("┲━━━┱", "╄━━━╃"),
            (_, y) if y + 1 == self.size() => ("╆━━━╅", "┺━━━┹"),
            (0, _) => ("┢━━━╅", "┡━━━╃"),
            (x, _) if x + 1 == self.size() => ("╆━━━┪", "╄━━━┩"),
            _ => ("╆━━━╅", "╄━━━╃"),
        };
        buffer.set_str(top, &Vec2::new(sel_x, sel_y));
        buffer.set_str(bottom, &Vec2::new(sel_x, sel_y + 2));
        buffer.set_val('┃', &Vec2::new(sel_x, sel_y + 1));
        buffer.set_val('┃', &Vec2::new(sel_x + 4, sel_y + 1));
    }

    /// Renders cells
    fn render_cells(&self, buffer: &mut Buffer) {
        let mut coords = Vec2::new(buffer.x() + 2, buffer.y() + 1);
        let mut id = 0;
        for _ in 0..self.size() {
            for _ in 0..self.size() {
                let cell = self[id];
                match cell.value() {
                    0 => {}
                    val if cell.enabled() => {
                        buffer.set_str(val.to_string(), &coords)
                    }
                    val => {
                        buffer.set_str_styled(
                            val.to_string(),
                            &coords,
                            Style::new().modifier(Modifier::BOLD),
                        );
                    }
                }
                id += 1;
                coords.x += 4;
            }
            coords.y += 2;
            coords.x = buffer.x() + 2;
        }
    }

    /// Renders outer borders
    fn render_outer(&self, buffer: &mut Buffer) {
        let bottom = self.size() * 2;
        let right = self.size() * 4;

        buffer.set_str_styled(
            "───┬".repeat(self.size()),
            &Vec2::new(buffer.x() + 1, buffer.y()),
            Style::new().fg(Color::Gray),
        );
        buffer.set_str_styled(
            "───┴".repeat(self.size()),
            &Vec2::new(buffer.x() + 1, buffer.y() + bottom),
            Style::new().fg(Color::Gray),
        );

        let mut leftc = Vec2::new(buffer.x(), buffer.y() + 1);
        let mut rightc = Vec2::new(buffer.x() + right, buffer.y() + 1);
        for _ in buffer.y()..buffer.y() + self.size() {
            Board::border_part('│', buffer, &leftc);
            leftc.y += 1;
            Board::border_part('├', buffer, &leftc);
            leftc.y += 1;

            Board::border_part('│', buffer, &rightc);
            rightc.y += 1;
            Board::border_part('┤', buffer, &rightc);
            rightc.y += 1;
        }

        let mut pos = buffer.pos().clone();
        Board::border_part('┌', buffer, &pos);
        pos.x += right;
        Board::border_part('┐', buffer, &pos);
        pos.y += bottom;
        Board::border_part('┘', buffer, &pos);
        pos.x -= right;
        Board::border_part('└', buffer, &pos);
    }

    /// Renders inner borders
    fn render_inner(&self, buffer: &mut Buffer) {
        let line = "───┼".repeat(self.size());
        for y in 1..self.size() {
            buffer.set_str_styled(
                &line,
                &Vec2::new(buffer.x() + 1, buffer.y() + y * 2),
                Style::new().fg(Color::Gray),
            );
        }

        let line = "   │".repeat(self.size());
        for y in 0..self.size() {
            buffer.set_str_styled(
                &line,
                &Vec2::new(buffer.x() + 1, buffer.y() + y * 2 + 1),
                Style::new().fg(Color::Gray),
            )
        }
    }

    fn render_hor_conds(&self, buffer: &mut Buffer) {
        let size = Vec2::new(self.size().saturating_sub(1), self.size());
        for y in 0..size.y {
            for x in 0..size.x {
                let c = match self.hor_conds[x + y * size.x] {
                    Some(true) => '>',
                    Some(false) => '<',
                    _ => continue,
                };
                buffer.set_val(
                    c,
                    &Vec2::new(buffer.x() + x * 4 + 4, buffer.y() + y * 2 + 1),
                );
            }
        }
    }

    fn render_ver_conds(&self, buffer: &mut Buffer) {
        let size = Vec2::new(self.size(), self.size().saturating_sub(1));
        for y in 0..size.y {
            for x in 0..size.x {
                let c = match self.ver_conds[x + y * size.x] {
                    Some(true) => '∨',
                    Some(false) => '∧',
                    _ => continue,
                };
                buffer.set_val(
                    c,
                    &Vec2::new(buffer.x() + x * 4 + 2, buffer.y() + y * 2 + 2),
                );
            }
        }
    }

    /// Renders part of the border
    fn border_part(val: char, buffer: &mut Buffer, pos: &Vec2) {
        buffer.set_val(val, pos);
        buffer.set_fg(Color::Gray, pos);
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
