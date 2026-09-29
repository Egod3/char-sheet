use crate::io::BufReader;
use clap::Parser;
use classes::CharSheet;
use ratatui::layout::Rect;
use std::fs::File;
use std::process::Command;
use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
            MouseButton, MouseEventKind,
        },
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
//mod classes;
use classes::parse_char_sheet;
mod ui;
use crate::{
    app::{
        App, CurrentScreen, HealthHover, HealthView, InspirationView, RestHover, RestView,
        ViewState,
    },
    ui::ui,
};

#[derive(Parser, Debug)]
struct Args {
    /// json_file to load instead of loading the default from "resource/default_sheet.json"
    #[arg(short, long)]
    json_file: Option<String>, // Truly optional

    /// The version of the app based on the git tag/version
    #[arg(short, long)]
    version: bool, // Truly optional
}

#[allow(clippy::needless_late_init)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.version {
        let version = Command::new("git")
            .args(["describe", "--tags", "--long", "--dirty=-modified"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        println!("git version:{}", version);
        return Ok(());
    }

    let mut json_file: String = "".to_string();
    let mut json_file_provided = false;
    if let Some(in_json_name) = args.json_file {
        //println!("Value for json_file:{}", in_json_name);

        json_file = in_json_name;
        json_file_provided = true;
    }

    let mut view_state: ViewState = ViewState {
        health: HealthView {
            minus_rect: Rect::new(0, 0, 0, 0),
            plus_rect: Rect::new(0, 0, 0, 0),
            hover: HealthHover::None,
        },
        rest: RestView {
            short_rest_rect: Rect::new(3, 2, 2, 0),
            long_rest_rect: Rect::new(10, 7, 7, 1),
            hover: RestHover::None,
        },
        inspiration: InspirationView {
            inspiration_toggle: Rect::new(0, 0, 0, 0),
        },
    };

    let file = match File::open(&json_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file: {err}");
            return Ok(());
        }
    };

    let reader = BufReader::new(file);
    let char_sheet: CharSheet;

    match parse_char_sheet(reader) {
        Ok(character) => {
            char_sheet = character;
        }
        Err(err) => {
            eprintln!("Failed to parse JSON file (it may be malformed): {err}");
            return Ok(());
        }
    }

    let mut app;
    if json_file_provided {
        // create app and run it
        app = App::new(json_file.to_string(), char_sheet);
    } else {
        // create app and run it
        app = App::new(
            "resources/default_sheet.json".to_string(),
            CharSheet::default(),
        );
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app, &mut view_state);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn rect_contains(rect: Rect, x: u16, y: u16) -> bool {
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}

#[derive(PartialEq)]
enum Action {
    Quit,
    HpIncrease,
    HpDecrease,
    ShortRest,
    LongRest,
    InspirationToggle,
    NextTab,
    PrevTab,
    EditChunk, // use w/ "selected_chunk" state and we can represent editing each chunk we have
    None,
}

fn handle_event(event: Event, view_state: &mut ViewState) -> Action {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') => {
            Action::Quit
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('l') => {
            Action::NextTab
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('h') => {
            Action::PrevTab
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e') => {
            Action::EditChunk
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('+') => {
            Action::HpIncrease
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('-') => {
            Action::HpDecrease
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('i') => {
            Action::InspirationToggle
        }

        Event::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Up(MouseButton::Left)) => {
            if rect_contains(view_state.health.minus_rect, mouse.column, mouse.row) {
                view_state.health.hover = HealthHover::Minus;
                Action::HpDecrease
            } else if rect_contains(view_state.health.plus_rect, mouse.column, mouse.row) {
                view_state.health.hover = HealthHover::Plus;
                Action::HpIncrease
            } else if rect_contains(view_state.rest.short_rest_rect, mouse.column, mouse.row) {
                view_state.rest.hover = RestHover::Long;
                Action::ShortRest
            } else if rect_contains(view_state.rest.long_rest_rect, mouse.column, mouse.row) {
                view_state.rest.hover = RestHover::Long;
                Action::LongRest
            } else if rect_contains(
                view_state.inspiration.inspiration_toggle,
                mouse.column,
                mouse.row,
            ) {
                Action::InspirationToggle
            } else {
                view_state.health.hover = HealthHover::None;
                Action::None
            }
        }
        _ => Action::None,
    }
}

fn apply_action(app: &mut App, action: &Action) -> bool {
    match action {
        Action::NextTab => app.next_tab(),
        Action::PrevTab => app.previous_tab(),
        Action::HpIncrease => app.char_sheet.health.increase(),
        Action::HpDecrease => {
            app.char_sheet.health.decrease();
        }
        Action::ShortRest => {
            app.char_sheet.health.short_rest();
        }
        Action::LongRest => {
            app.char_sheet.health.long_rest();
        }
        Action::InspirationToggle => {
            app.char_sheet.statistics.insp_toggle();
        }
        Action::Quit => return false,
        // TODO: add support to edit each of the text area's that make sense to allow the user to
        // edit
        Action::EditChunk => {
            //println!("edit chunk seleted!");
        }
        Action::None => {}
    }
    true
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    view_state: &mut ViewState,
) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app, view_state)).unwrap();
        let timeout = std::time::Duration::from_millis(250);

        if event::poll(timeout)? {
            let action: Action = handle_event(event::read()?, view_state);
            let quit: bool = apply_action(app, &action);
            if !quit {
                app.current_screen = CurrentScreen::Exiting;
                break Ok(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
    };
    use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    use ratatui::layout::Rect;

    #[test]
    fn apply_hp_increase_action() {
        let mut app = App::new("resources/default_sheet.json".to_string());
        app.save_file = false;
        let _ = apply_action(&mut app, &Action::HpIncrease);
        assert_eq!(app.char_sheet.health.current_hp, 1);
    }

    #[test]
    fn apply_hp_decrease_action() {
        let mut app = App::new("resources/default_sheet.json".to_string());
        app.save_file = false;
        let _ = apply_action(&mut app, &Action::HpIncrease);
        assert_eq!(app.char_sheet.health.current_hp, 1);
    }

    #[test]
    fn pressing_q_returns_quit_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::Quit));
    }

    #[test]
    fn clicking_plus_returns_hp_increase() {
        let mut view_state: ViewState = ViewState {
            health: HealthView {
                minus_rect: Rect::new(0, 0, 0, 0),
                plus_rect: Rect::new(10, 5, 5, 1),
                hover: HealthHover::None,
            },
            rest: RestView {
                short_rest_rect: Rect::new(3, 2, 2, 0),
                long_rest_rect: Rect::new(10, 7, 7, 1),
                hover: RestHover::None,
            },
            inspiration: InspirationView {
                inspiration_toggle: Rect::new(0, 0, 0, 0),
            },
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 12,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::HpIncrease));
    }

    #[test]
    fn clicking_plus_returns_hp_decrease() {
        let mut view_state: ViewState = ViewState {
            health: HealthView {
                minus_rect: Rect::new(10, 5, 5, 1),
                plus_rect: Rect::new(0, 0, 0, 0),
                short_rest_rect: Rect::new(3, 2, 2, 0),
                long_rest_rect: Rect::new(10, 7, 7, 1),
                hover: HealthHover::None,
            },
            inspiration: InspirationView {
                inspiration_toggle: Rect::new(0, 0, 0, 0),
            },
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 12,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::HpDecrease));
    }

    #[test]
    fn clicking_inspiration_toggle() {
        let mut view_state: ViewState = ViewState {
            health: HealthView {
                minus_rect: Rect::new(0, 0, 0, 0),
                plus_rect: Rect::new(0, 0, 0, 0),
                hover: HealthHover::None,
            },
            rest: RestView {
                short_rest_rect: Rect::new(3, 2, 2, 0),
                long_rest_rect: Rect::new(10, 7, 7, 1),
                hover: RestHover::None,
            },
            inspiration: InspirationView {
                inspiration_toggle: Rect::new(10, 5, 5, 1),
            },
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 12,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::InspirationToggle));
    }
}
