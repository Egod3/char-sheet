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

#[derive(PartialEq)]
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

fn apply_action(app: &mut App, action: &Action) -> bool {
    match action {
        Action::HpIncrease => app.char_sheet.health.increase(),
        Action::HpDecrease => {
            app.char_sheet.health.decrease();
        }
        Action::Quit => return false,
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
            let action: Action = handle_event(event::read()?, &mut view_state.health);
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

        let mut view = HealthView::default();
        let action = handle_event(event, &mut view);

        assert!(matches!(action, Action::Quit));
    }

    #[test]
    fn clicking_plus_returns_hp_increase() {
        let mut view = HealthView {
            minus_rect: Rect::new(0, 0, 0, 0),
            plus_rect: Rect::new(0, 0, 0, 0),
            hover: Hover::None,
        };
        view.plus_rect = Rect {
            x: 10,
            y: 5,
            width: 5,
            height: 1,
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 12,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view);

        assert!(matches!(action, Action::HpIncrease));
    }

    #[test]
    fn clicking_plus_returns_hp_decrease() {
        let mut view = HealthView {
            minus_rect: Rect::new(0, 0, 0, 0),
            plus_rect: Rect::new(0, 0, 0, 0),
            hover: Hover::None,
        };
        view.minus_rect = Rect {
            x: 10,
            y: 5,
            width: 5,
            height: 1,
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 12,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view);

        assert!(matches!(action, Action::HpDecrease));
    }
}
