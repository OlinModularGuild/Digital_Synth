use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    execute,

};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::collections::LinkedList;
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs, Dataset, Axis, Chart, GraphType
    },
    Terminal,
    symbols,
};

enum Event<I> {
    Input(I),
    Tick,
}

struct Wave {
    freq: f32,
    module: i16, //1 - base oscillator, 2 - amplifier, 3 - filter,
    form: i16, // 1 - sine, 2 - saw, 3 - square
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // test stuff with a linked list
    let mut ls: LinkedList<Wave> = LinkedList::new();
    ls.push_back(Wave { freq: 444.0, module: 1, form: 1 });
    ls.push_back(Wave { freq: 20.0, module: 2, form: 3});

    // setup terminal
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    // spawn thread to accept keyboard input
    thread::spawn(move || {
        let mut last_time = Instant::now();
        loop{
            let timeout=tick_rate
                .checked_sub(last_time.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            
            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_time.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_time = Instant::now();
                }
            }

        }
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    
    loop{
        terminal.draw(|rect| {
            let size = rect.size();
            let vsplit = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                ).split(size);

            let desc=Paragraph::new("a - create new wave | e - edit wave | arrows - navigate")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Digital Synth")
                        .border_type(BorderType::Plain),
                );
            rect.render_widget(desc, vsplit[0]);
            let hsplit = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                )
                .split(vsplit[1]);
            let (left, right) = draw_waves(&mut ls);
            rect.render_widget(left, hsplit[0]);
            rect.render_widget(right, hsplit[1]);
        });
    }

    // restore terminal
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw_waves(ls: &mut LinkedList<Wave>) -> (List, Chart){
    let mut items = ls.iter();
    let listt:Vec<ListItem> = items
        .map(|wave| { ListItem::new(print_wave(wave)) })
        .collect();

    let list = List::new(listt)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    let dataset = make_dataset();
    let chart = Chart::new(dataset)
        .block(Block::default().title("Chart"))
        .x_axis(Axis::default()
            .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect())); 
    (list, chart)
}

fn make_dataset() -> Vec<Dataset<'static>>{
    // take in the signal and return it as a datset
    // this is a placeholder until we get actual signals
    let datasets = vec![
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Cyan))
            .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];

    datasets
}

fn print_wave(wave: &Wave) -> String {
    format!("{module}, {freq} hz", 
        module = match wave.module {
            1 => "Oscillator",
            2 => "Amplifier",
            3 => "Filter",
            _ => "Invalid Wave",
        },
        freq = wave.freq)
}