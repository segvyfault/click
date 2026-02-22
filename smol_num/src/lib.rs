/*!
 * small num fonts
 * like that one tty-clock font you know like
 * font8x8 SUCKS. its too big and ugly
 *
 * smol_num is used to draw small numbers
 * if it wasnt obvious enough from the name
 */
mod table;

use ratatui::prelude::*;
use ratatui::{style::Color, widgets::Widget};
use crate::table::{NumberVariant, PointsToString};

pub struct BigNumber<'a> {
    num: String,
    color: &'a Color,
}

impl<'a> BigNumber<'a> {
    pub fn new(num: impl Into<String>, color: &'a Color) -> Self {
        let num = num.into().replace(char::is_alphabetic, "");
        Self { num, color }
    }

    pub fn width(&self) -> usize {
        let len = self.num.len();
        len * 6 + len
    }

    pub fn draw_line(&self, line: u8) -> String {
        let mut buffer = String::new();

        self.num.chars()
            .for_each(|c| if let Ok(num) = NumberVariant::try_from(c) {
                let is_colon = num == NumberVariant::Colon;

                // dont ask me why, but tty-clock does it
                // and i think it actually looks better this way
                if is_colon {
                    buffer.pop();
                }

                let points = num.get_points(line as usize);
                buffer.push_str(&String::from_points(points));

                if !is_colon {
                    buffer.push(' ');
                }
            });

        buffer
    }
}

impl Widget for BigNumber<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (0u8..5).for_each(|line| {
            let string = self.draw_line(line);
            buf.set_string(area.x, area.y + line as u16, string, Style::default().fg(*self.color));
        });
    }
}

#[test]
fn text_width() {
    let num = BigNumber::new("123", &Color::White);
    let len = num.num.len();

    let smart = len * 6 + len;

    let mut stupid = 0;
    (0..len)
        .for_each(|i| stupid += i * 6 + 1);

    assert_eq!(smart, 21);
    assert_eq!(stupid, 21);
}

#[test]
fn buffer_lines() {
    let num = BigNumber::new("124", &Color::White);

    assert_eq!(
        num.draw_line(0).trim_start(),
        "██ ██████ ██  ██ "
    );
    assert_eq!(
        num.draw_line(1).trim_start(),
        "██     ██ ██  ██ "
    );
    assert_eq!(
        num.draw_line(2).trim_start(),
        "██ ██████ ██████ "
    );
    assert_eq!(
        num.draw_line(3).trim_start(),
        "██ ██         ██ "
    );
    assert_eq!(
        num.draw_line(4).trim_start(),
        "██ ██████     ██ "
    );
}
