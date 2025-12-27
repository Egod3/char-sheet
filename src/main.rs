// use serde_json;
//use std::{error::Error, fs::File, io, io::Read, io::Result};
use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

//fn main() -> Result<()> {
// println!("welcome to char-sheet");

// // Exmple sheet that is based on DND 5e Cromwell Windscream a Path of the Giant Barbarian
// let mut file = File::open("resources/character_sheet.json").unwrap();
// let mut buff = String::new();
// file.read_to_string(&mut buff).unwrap();
// let char_sheet: CharSheet = serde_json::from_str(&buff).unwrap();
// let information: Information = char_sheet.information;
// let statistics: Statistics = char_sheet.statistics;
// let saving_throws: SavingThrows = char_sheet.saving_throws;
// let skills: Skills = char_sheet.skills;
// let proficiencies_and_language: ProficienciesAndLanguage =
//     char_sheet.proficiencies_and_language;
// let health: Health = char_sheet.health;

// println!("information.character_name: {}", information.character_name);
// println!("information.class: {}", information.class);
// println!("information.level: {}", information.level);
// println!("information.background: {}", information.background);
// println!("information.player_name: {}", information.player_name);
// println!("information.race: {}", information.race);
// println!("information.alignment: {}", information.alignment);
// println!("information.experience: {}", information.experience);

// println!("statistics.strength: {}", statistics.strength);

// println!(
//     "saving_throws.strength_proficent: {}",
//     saving_throws.strength_proficent
// );

// println!("skills.acrobatics: {}", skills.acrobatics);

// println!(
//     "proficiencies_and_language.languages_known: {}",
//     proficiencies_and_language.languages_known
// );

// println!("health.armor_class: {}", health.armor_class);

// Ok(())
// } # Original main()

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new("resources/character_sheet.json".to_string());
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.save_key_value();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.pop();
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Tab => {
                            app.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.push(value);
                                    }
                                }
                            }
                        }
                        // END: character_editing
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // END: event_poll
    }
}
