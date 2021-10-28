use rppal::{hal::Delay, i2c::I2c};
use std::error::Error;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem};
use tui::{symbols, Terminal};

use xgzp6859d::Xgzp6859d;

const DATA_LEN: usize = 200;

fn main() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;
    let delay = Delay::new();

    let mut sensor = Xgzp6859d::new(i2c, delay)?;
    let mut buffer = vec![];

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().ok();

    for _ in 0..6000 {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let pressure = sensor.read_pressure().unwrap();
            let _ = &buffer.push(pressure);
            if buffer.len() > DATA_LEN {
                let _ = &buffer.remove(0);
            }

            let dataset = &buffer
                .iter()
                .enumerate()
                .map(|(i, p)| (i as f64, *p as f64))
                .collect::<Vec<(f64, f64)>>();

            let datasets = vec![Dataset::default()
                .name("pressure(Pa)")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&dataset)];

            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(vec![
                            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("100"),
                            Span::styled("200", Style::default().add_modifier(Modifier::BOLD)),
                        ])
                        .bounds([0.0, 200.0]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(vec![
                            Span::styled("-75KPa", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("-40KPa"),
                            Span::styled("5KPa", Style::default().add_modifier(Modifier::BOLD)),
                        ])
                        .bounds([-75_000.0, 5_000.0]),
                );
            f.render_widget(chart, chunks[1]);

            let len = buffer.len();
            let items = if len > 40 {
                buffer[len - 40..len]
                    .iter()
                    .map(|p| ListItem::new(format!("{}", p)))
                    .collect::<Vec<ListItem>>()
            } else {
                buffer
                    .iter()
                    .map(|p| ListItem::new(format!("{}", p)))
                    .collect::<Vec<ListItem>>()
            };

            let list = List::new(items)
                .block(Block::default().title("Pressure(Pa)").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(list, chunks[0]);

            let block = Block::default()
                .title("Pressure Data")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
    }

    terminal.clear().ok();

    Ok(())
}
