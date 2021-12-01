/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use tui::widgets::canvas::{Canvas, Rectangle, Line};
use std::ops::Add;
use hang::Triangle;
use tui::layout::Alignment;
use std::collections::HashSet;

const TRIANGLE1: Triangle =  Triangle {
    p1: (10.0, 10.0),
    p2: (45.0, 45.0),
    p3: (50.0, 10.0),
    color: Color::Yellow,
};

const TRIANGLE2: Triangle =  Triangle {
    p1: (55.0, 45.0),
    p2: (90.0, 50.0),
    p3: (90.0, 10.0),
    color: Color::Yellow,
};

const TRIANGLE3: Triangle =  Triangle {
    p2: (90.0, 90.0),
    p1: (50.0, 90.0),
    p3: (55.0, 55.0),
    color: Color::Yellow,
};

const TRIANGLE4: Triangle =  Triangle {
    p1: (10.0, 90.0),
    p2: (45.0, 55.0),
    p3: (10.0, 50.0),
    color: Color::Yellow,
};

const CENTER: Rectangle =  Rectangle {
    x: 45.0,
    y: 45.0,
    width: 10.0,
    color: Color::Yellow,
    height: 10.0
};

const LINE_1: Line =  Line {
    x1: 45.0,
    y1: 45.0,
    x2: 55.0,
    y2: 55.0,
    color: Color::LightGreen
};
const LINE_2: Line =  Line {
    x1: 45.0,
    y1: 55.0,
    x2: 55.0,
    y2: 45.0,
    color: Color::LightGreen
};


/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    word: String,
    fails: i8,
    hidden_letters: HashSet<char>
}

impl Default for App {
    fn default() -> App {
        App::new(String::from("Masupilami"))
    }
}

impl App {
    fn new(word: String) -> App {
        let upperword = word.to_uppercase();
        App {
            input: String::new(),
            hidden_letters: upperword.chars().collect::<HashSet<_>>(),
            word: upperword,
            fails: 0,
        }
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if c.is_ascii_alphabetic() {
                        let upper_c = c.to_ascii_uppercase();
                        if !app.input.contains(&*upper_c.to_string()) && app.hidden_letters.contains(&upper_c) {
                            app.input = app.input.add(&*upper_c.to_string());
                            app.hidden_letters.remove(&upper_c);
                        }
                        else {
                            app.fails = app.fails + 1;
                        }
                    }

                }
                KeyCode::Esc => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
                .as_ref(),
        )
        .split(f.size());

    let (msg, style) =  (
            vec![
                Span::raw("Press "),
                Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit. "),
                Span::raw("Type to play hangman."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        );
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Typed letters"));
    f.render_widget(input, chunks[1]);
    f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            );

    let word = Paragraph::new(app.word.as_ref())
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(format!("Current State of hangman [{}]",app.hidden_letters.iter().fold(String::new(), |acc, x| acc + &*x.to_string()))));
    f.render_widget(word, chunks[2]);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(format!("Hang fan. Wrong letters [{}]",app.fails)))
        .paint(|ctx| {
            ctx.draw(&TRIANGLE1);
            ctx.draw(&TRIANGLE2);
            ctx.draw(&TRIANGLE3);
            ctx.draw(&TRIANGLE4);
            ctx.draw(&CENTER);
            ctx.draw(&LINE_1);
            ctx.draw(&LINE_2);
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0]);
    f.render_widget(canvas, chunks[3]);
}
