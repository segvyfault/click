mod clock;
mod config;
mod events;

use crate::{clock::DisplayClock, config::Config, events::EventHandler};
use ratatui::{
    layout::{Constraint, Offset}, style::Stylize, text::Text
};
use smol_num::BigNumber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().parse_cli().build();
    let display = config.display_config();

    let (handler, should_quit) = EventHandler::new(display.clone());
    let mut clock = DisplayClock::new(display.clone());

    ratatui::run(move |terminal| {
        loop {
            clock.update();

            terminal.draw(|frame| {
                if let Ok(display) = display.read() {
                    let num = BigNumber::new(clock.time(), config.fg());
                    let num_width = num.width() as u16;

                    let area =
                        if display.centerize_text {
                            frame.area().centered(
                                Constraint::Length(num_width),
                                Constraint::Length(5)
                            )
                        }
                        else { frame.area().offset(Offset::new(0, 1)) };

                    frame.render_widget(num, area);

                    if !display.hide_date {
                        let text = Text::raw(clock.date())
                            .fg(*config.fg());

                        // makes the text look a bit more centered :3
                        let num_width_offset = if display.show_seconds { 1 } else { 0 };
                        let text_area = area
                            .offset(Offset::new(
                                (num_width as i32 / 2 - num_width_offset) - text.width() as i32 / 2,
                                6
                            ));

                        frame.render_widget(text, text_area);
                    }
                }
            })?;

            handler.handle_events()?;

            if let Ok(quit) = should_quit.read() &&
                *quit == true
            {
                break;
            }
        }
        Ok(())
    })
}
