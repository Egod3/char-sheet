use ratatui::layout::Rect;
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
mod ui;
use crate::{
    app::{App, CurrentScreen, HealthView, Hover, ViewState},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut view_state: ViewState = ViewState {
        health: HealthView {
            minus_rect: Rect::new(0, 0, 0, 0),
            plus_rect: Rect::new(0, 0, 0, 0),
            hover: Hover::None,
        },
    };

    // create app and run it
    let mut app = App::new("resources/character_sheet.json".to_string());
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

enum Action {
    Quit,
    HpIncrease,
    HpDecrease,
    None,
}

fn handle_event(event: Event, view: &mut HealthView) -> Action {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') => {
            Action::Quit
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('+') => {
            Action::HpIncrease
        }

        Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('-') => {
            Action::HpDecrease
        }

        Event::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Up(MouseButton::Left)) => {
            if rect_contains(view.minus_rect, mouse.column, mouse.row) {
                view.hover = Hover::Minus;
                Action::HpDecrease
            } else if rect_contains(view.plus_rect, mouse.column, mouse.row) {
                view.hover = Hover::Plus;
                Action::HpIncrease
            } else {
                view.hover = Hover::None;
                Action::None
            }
        }
        _ => Action::None,
    }
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
            match handle_event(event::read()?, &mut view_state.health) {
                Action::Quit => {
                    app.current_screen = CurrentScreen::Exiting;
                    break Ok(false);
                }

                Action::HpIncrease => {
                    app.char_sheet.health.current_hp = (app.char_sheet.health.current_hp + 1)
                        .min(app.char_sheet.health.maximum_hp);
                }

                Action::HpDecrease => {
                    if app.char_sheet.health.current_hp >= 1 {
                        app.char_sheet.health.current_hp = app.char_sheet.health.current_hp - 1;
                    } else {
                        app.char_sheet.health.current_hp = 0;
                    }
                }

                Action::None => {}
            }
        }
    }
}
