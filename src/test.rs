#[cfg(test)]
mod tests {
    use crate::app::{
        App, HealthHover, HealthView, InspirationView, RestHover, RestView, ViewState,
    };
    use crate::ui::{SelectedTabBackGround, SelectedTextColor};
    use crate::{apply_action, handle_event, Action};
    use classes::parse_char_sheet;
    use ratatui::crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
    };
    use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    use ratatui::layout::Rect;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn apply_quit_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let ret = apply_action(&mut app, &Action::Quit);
                assert_eq!(ret, false);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_hp_increase_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::HpIncrease);
                assert_eq!(app.char_sheet.health.current_hp, 1);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_hp_decrease_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = 2;
                let _ = apply_action(&mut app, &Action::HpDecrease);
                assert_eq!(app.char_sheet.health.current_hp, 1);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_hp_increase_action_test_temp_hp() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = app.char_sheet.health.maximum_hp;
                let _ = apply_action(&mut app, &Action::HpIncrease);
                let _ = apply_action(&mut app, &Action::HpIncrease);
                let _ = apply_action(&mut app, &Action::HpIncrease);
                let _ = apply_action(&mut app, &Action::HpIncrease);
                assert_eq!(app.char_sheet.health.temporary_hp, 4);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_hp_decrease_action_test_temp_hp() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = app.char_sheet.health.maximum_hp;
                app.char_sheet.health.temporary_hp = 3;
                let _ = apply_action(&mut app, &Action::HpDecrease);
                let _ = apply_action(&mut app, &Action::HpDecrease);
                let _ = apply_action(&mut app, &Action::HpDecrease);
                assert_eq!(app.char_sheet.health.temporary_hp, 0);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_next_tab_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::NextTab);
                assert_eq!(app.sel_tab_bck_grnd, SelectedTabBackGround::Background);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_prev_tab_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::NextTab);
                let _ = apply_action(&mut app, &Action::NextTab);
                let _ = apply_action(&mut app, &Action::PrevTab);
                let _ = apply_action(&mut app, &Action::PrevTab);
                assert_eq!(app.sel_tab_bck_grnd, SelectedTabBackGround::ProfLang);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_prev_tab_action_test_feat_trait() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::NextTab);
                let _ = apply_action(&mut app, &Action::NextTab);
                assert_eq!(app.sel_tab_bck_grnd, SelectedTabBackGround::FeatTrait);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_char_classes_scroll_up_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_class_para.max_scroll_lines = 5;
                app.sel_tab_bck_grnd = SelectedTabBackGround::FeatTrait;
                let _ = apply_action(&mut app, &Action::ScrollUp);
                assert_eq!(app.char_class_para.vert_scroll_offset, 1);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_char_classes_scroll_down_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.sel_tab_bck_grnd = SelectedTabBackGround::FeatTrait;
                let _ = apply_action(&mut app, &Action::ScrollUp);
                let _ = apply_action(&mut app, &Action::ScrollUp);
                let _ = apply_action(&mut app, &Action::ScrollDown);
                let _ = apply_action(&mut app, &Action::ScrollDown);
                let _ = apply_action(&mut app, &Action::ScrollDown);
                assert_eq!(app.char_class_para.vert_scroll_offset, 0);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_toggle_color_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                assert_eq!(app.text_color, SelectedTextColor::MagentaYellow);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_toggle_color_action_one() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                assert_eq!(app.text_color, SelectedTextColor::DarkGrayLightMagenta);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_toggle_color_action_two() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                assert_eq!(app.text_color, SelectedTextColor::LightYellowCyan);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_toggle_color_action_three() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                let _ = apply_action(&mut app, &Action::ToggleTextColor);
                assert_eq!(app.text_color, SelectedTextColor::GreenLightRed);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_show_help_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::ShowHelp);
                assert!(app.show_help);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_hide_help_action() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                let _ = apply_action(&mut app, &Action::ShowHelp);
                let _ = apply_action(&mut app, &Action::HideHelp);
                assert!(!app.show_help);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_death_save_action_back_to_life() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = 0;
                let _ = apply_action(&mut app, &Action::DeathSave);
                assert_eq!(app.char_sheet.health.death_save_saves, 1);
                let _ = apply_action(&mut app, &Action::DeathSave);
                assert_eq!(app.char_sheet.health.death_save_saves, 2);
                let _ = apply_action(&mut app, &Action::DeathSave);
                assert_eq!(app.char_sheet.health.death_save_saves, 0);
                assert_eq!(app.char_sheet.health.current_hp, 1);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_hp_increase_action_when_dead_back_to_life() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = 0;
                let _ = apply_action(&mut app, &Action::HpIncrease);
                assert_eq!(app.char_sheet.health.death_save_saves, 0);
                assert_eq!(app.char_sheet.health.current_hp, 1);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_death_fail_action_to_death() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = 0;
                let _ = apply_action(&mut app, &Action::DeathFail);
                assert_eq!(app.char_sheet.health.death_save_fails, 1);
                let _ = apply_action(&mut app, &Action::DeathFail);
                assert_eq!(app.char_sheet.health.death_save_fails, 2);
                let _ = apply_action(&mut app, &Action::DeathFail);
                assert_eq!(app.char_sheet.health.death_save_fails, 3);
                assert_eq!(app.char_sheet.health.current_hp, 0);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_death_save_action_when_alive() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = 5;
                let _ = apply_action(&mut app, &Action::DeathSave);
                assert_eq!(app.char_sheet.health.death_save_saves, 0);
                let _ = apply_action(&mut app, &Action::DeathSave);
                assert_eq!(app.char_sheet.health.death_save_saves, 0);
                let _ = apply_action(&mut app, &Action::DeathSave);
                assert_eq!(app.char_sheet.health.death_save_saves, 0);
                assert_eq!(app.char_sheet.health.current_hp, 5);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn apply_death_fail_action_when_alive() {
        let file = match File::open("classes/resources/default_sheet.json") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file: {err}");
                return;
            }
        };
        let reader = BufReader::new(file);

        match parse_char_sheet(reader) {
            Ok(character) => {
                let mut app = App::new(
                    "classes/resources/default_sheet.json".to_string(),
                    character,
                );
                app.save_file = false;
                app.char_sheet.health.current_hp = 5;
                let _ = apply_action(&mut app, &Action::DeathFail);
                assert_eq!(app.char_sheet.health.death_save_fails, 0);
                let _ = apply_action(&mut app, &Action::DeathFail);
                assert_eq!(app.char_sheet.health.death_save_fails, 0);
                let _ = apply_action(&mut app, &Action::DeathFail);
                assert_eq!(app.char_sheet.health.death_save_fails, 0);
                assert_eq!(app.char_sheet.health.current_hp, 5);
            }
            Err(err) => {
                eprintln!("Failed to parse JSON file (it may be malformed): {err}");
                assert_eq!(false, true);
            }
        }
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
    fn pressing_l_returns_prev_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::NextTab));
    }

    #[test]
    fn pressing_h_returns_prev_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::PrevTab));
    }

    #[test]
    fn pressing_i_returns_inspiration_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::InspirationToggle));
    }

    #[test]
    fn pressing_plus_returns_hp_increase_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('+'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::HpIncrease));
    }

    #[test]
    fn pressing_minus_returns_hp_decrease_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('-'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::HpDecrease));
    }

    #[test]
    fn pressing_e_returns_edit_chunk_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::EditChunk));
    }

    #[test]
    fn pressing_j_returns_char_class_scroll_up_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::ScrollUp));
    }

    #[test]
    fn pressing_k_returns_char_class_scroll_up_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::ScrollDown));
    }

    #[test]
    fn pressing_c_returns_toggle_text_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::ToggleTextColor));
    }

    #[test]
    fn pressing_s_returns_death_save_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::DeathSave));
    }

    #[test]
    fn pressing_f_returns_death_save_action() {
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('f'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
        });

        let mut view_state = ViewState::default();
        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::DeathFail));
    }

    #[test]
    fn clicking_plus_returns_hp_increase() {
        let mut view_state: ViewState = ViewState {
            health: HealthView {
                //  Rect::new(x, y, width, height)
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
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::HpIncrease));
    }

    #[test]
    fn clicking_minus_returns_hp_decrease() {
        let mut view_state: ViewState = ViewState {
            health: HealthView {
                minus_rect: Rect::new(10, 5, 5, 1),
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

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 10,
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
                short_rest_rect: Rect::new(0, 0, 0, 0),
                long_rest_rect: Rect::new(0, 0, 0, 0),
                hover: RestHover::None,
            },
            inspiration: InspirationView {
                inspiration_toggle: Rect::new(10, 5, 5, 1),
            },
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::InspirationToggle));
    }

    #[test]
    fn clicking_short_rest_button() {
        let mut view_state: ViewState = ViewState {
            health: HealthView {
                minus_rect: Rect::new(0, 0, 0, 0),
                plus_rect: Rect::new(0, 0, 0, 0),
                hover: HealthHover::None,
            },
            rest: RestView {
                short_rest_rect: Rect::new(3, 2, 2, 1),
                long_rest_rect: Rect::new(10, 7, 7, 1),
                hover: RestHover::None,
            },
            inspiration: InspirationView {
                inspiration_toggle: Rect::new(0, 0, 0, 0),
            },
        };

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 3,
            row: 2,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::ShortRest));
    }

    #[test]
    fn clicking_long_rest_button() {
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

        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 10,
            row: 7,
            modifiers: KeyModifiers::NONE,
        });

        let action = handle_event(event, &mut view_state);

        assert!(matches!(action, Action::LongRest));
    }
}
