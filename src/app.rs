use crate::ui::SelectedTabBackGround;
use ratatui::layout::Rect;
//use ratatui::widgets::ListItem;
//use serde::{Deserialize, Serialize};
use classes::CharSheet;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Default)]
pub struct ViewState {
    pub health: HealthView,
    pub rest: RestView,
    pub inspiration: InspirationView,
}

#[derive(Default)]
pub enum HealthHover {
    Minus,
    Plus,
    #[default]
    None,
}

#[derive(Default)]
pub struct HealthView {
    pub minus_rect: Rect,
    pub plus_rect: Rect,
    pub hover: HealthHover,
}

#[derive(Default)]
#[allow(dead_code)]
pub enum RestHover {
    Short,
    Long,
    #[default]
    None,
}

#[derive(Default)]
pub struct RestView {
    pub short_rest_rect: Rect,
    pub long_rest_rect: Rect,
    pub hover: RestHover,
}

#[derive(Default)]
pub struct InspirationView {
    pub inspiration_toggle: Rect,
}

#[derive(Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    Exiting,
}

#[allow(dead_code)]
pub enum CurrEditInformation {
    CharacterName,
    Class,
    Level,
    Background,
    PlayerName,
    Race,
    Alignment,
    Experience,
    Value,
}

#[derive(Default)]
pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub char_sheet: CharSheet,
    pub json_file_name: String,
    pub save_file: bool,
    pub char_classes_scroll_offset: u16,
    // SelectedTab_BackGround or st_bg
    pub sel_tab_bck_grnd: SelectedTabBackGround,
}

impl App {
    pub fn new(json_file: String, char_sheet: CharSheet) -> App {
        App {
            current_screen: CurrentScreen::Main,
            char_sheet,
            json_file_name: json_file.clone(),
            char_classes_scroll_offset: 0,
            save_file: true,
            sel_tab_bck_grnd: SelectedTabBackGround::ProfLang,
        }
    }

    pub fn next_tab(&mut self) {
        self.sel_tab_bck_grnd = self.sel_tab_bck_grnd.next();
    }

    pub fn previous_tab(&mut self) {
        self.sel_tab_bck_grnd = self.sel_tab_bck_grnd.previous();
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if self.save_file {
            println!("Closing the file and writing it disk");
            let file = File::create(self.json_file_name.clone()).unwrap();
            let mut writer = BufWriter::new(file); // Use BufWriter for performance
            let _ = serde_json::to_writer_pretty(&mut writer, &self.char_sheet);

            writer.flush().unwrap();
        } else {
            println!("Not saving file since self.save_file is false");
        }
    }
}
