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
use tui::widgets::canvas::{Canvas, Context};
use std::ops::Add;
use hadvent::{GeneralSquare};
use tui::layout::Alignment;
use std::collections::{HashSet};
use itertools::sorted;
use structopt::StructOpt;

const STOMP: GeneralSquare =  GeneralSquare {
    p1: (40.0, 2.0),
    p2: (60.0, 2.0),
    p3: (55.0, 15.0),
    p4: (45.0, 15.0),
    color: Color::LightRed,
};

const TREE1: GeneralSquare =  GeneralSquare {
    p1: (20.0, 15.0),
    p2: (80.0, 15.0),
    p3: (65.0, 32.5),
    p4: (35.0, 32.5),
    color: Color::LightGreen,
};

const TREE2: GeneralSquare =  GeneralSquare {
    p1: (25.0, 32.5),
    p2: (75.0, 32.5),
    p3: (60.0, 50.0),
    p4: (40.0, 50.0),
    color: Color::LightGreen,
};

const TREE3: GeneralSquare =  GeneralSquare {
    p1: (30.0, 50.0),
    p2: (70.0, 50.0),
    p3: (55.0, 67.5),
    p4: (45.0, 67.5),
    color: Color::LightGreen,
};

const TREE4: GeneralSquare =  GeneralSquare {
    p1: (35.0, 67.5),
    p2: (65.0, 67.5),
    p3: (50.0, 85.0),
    p4: (50.0, 85.0),
    color: Color::LightGreen,
};


struct App {
    word: String,
    fails: i8,
    hidden_letters: HashSet<char>,
    typed_letters: HashSet<char>
}

impl Default for App {
    fn default() -> App {
        App::new(String::from("MerryChristmas"))
    }
}

impl App {
    fn new(word: String) -> App {
        let upper_word = word.to_uppercase();
        App {
            hidden_letters: upper_word.chars().collect::<HashSet<_>>(),
            word: upper_word,
            fails: 0,
            typed_letters: HashSet::new()
        }
    }

}

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    riddle: Option<String>,
}


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let args = Cli::from_args();

    // create app and run it
    let app = match args.riddle {
        Some(riddle) => App::new(riddle),
        None => App::default(),
    };
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
                    if c.is_ascii_alphabetic() && app.fails < 6 && !app.hidden_letters.is_empty() {
                        let upper_c = c.to_ascii_uppercase();
                        if !app.typed_letters.contains(&upper_c) && !app.hidden_letters.contains(&upper_c) {
                            app.fails = app.fails + 1;
                        }
                        app.hidden_letters.remove(&upper_c);
                        app.typed_letters.insert(upper_c);
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

    let (game_running, game_over, won, style) =  (
            vec![
                Span::raw("Press "),
                Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit. "),
                Span::raw("Type to play hangman."),
            ],
            vec![
                Span::styled("GAME OVER", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" ECS to exit. ")
            ],
            vec![
                Span::styled("You Won", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" ECS to exit. ")
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        );
    let mut text = if app.hidden_letters.is_empty() {
        Text::from(Spans::from(won))
    } else if app.fails >= 6 {
        Text::from(Spans::from(game_over))
    } else {
        Text::from(Spans::from(game_running))
    };


    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(sorted(app.typed_letters.iter()).fold(String::new(), |acc, x| acc + " "+ &*x.to_string()))
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Typed letters"));
    f.render_widget(input, chunks[1]);


    let rendered_word = app.word.clone().split("").fold(String::new(), |acc, s| acc.add(" ").add(s));
    let replaced_word = app.hidden_letters.iter().fold(rendered_word,|acc,c|  acc.replace(&*c.to_string(),"_"));

    let word = Paragraph::new(replaced_word)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(format!("Current State of hangman")));
    f.render_widget(word, chunks[2]);

    let mut canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(format!("Hang tree. Wrong letters [{}]",app.fails)));

    canvas = canvas.paint(|ctx :&mut Context| {
        match app.fails {
            0 => {},
            1 => ctx.draw(&STOMP),
            2 => {
                ctx.draw(&STOMP);
                ctx.draw(&TREE1);
            },
            3 => {
                ctx.draw(&STOMP);
                ctx.draw(&TREE1);
                ctx.draw(&TREE2);
            },
            4 => {
                ctx.draw(&STOMP);
                ctx.draw(&TREE1);
                ctx.draw(&TREE2);
                ctx.draw(&TREE3);
            },
            5 => {
                ctx.draw(&STOMP);
                ctx.draw(&TREE1);
                ctx.draw(&TREE2);
                ctx.draw(&TREE3);
                ctx.draw(&TREE4);
            },
            _ => {
                ctx.draw(&STOMP);
                ctx.draw(&TREE1);
                ctx.draw(&TREE2);
                ctx.draw(&TREE3);
                ctx.draw(&TREE4);
                ctx.print(45.0, 95.0, "**** Game Over ****", Color::Red);
            }
        }
    });

    canvas = canvas.x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0]);

    f.render_widget(canvas, chunks[3]);
}
