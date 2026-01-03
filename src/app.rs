use ratatui::layout::Rect;
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Information {
    pub character_name: String,
    pub class: String,
    pub level: u8,
    pub background: String,
    pub player_name: String,
    pub race: String,
    pub alignment: String,
    pub experience: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
// Modifiers will be calculated based on rules of the game
pub struct Statistics {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub inspiration: bool,
    pub proficiency_bonus: u8,
    pub passive_wisdom_perception: u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct SavingThrows {
    pub strength_proficent: bool,
    pub dexterity_proficent: bool,
    pub constitution_proficent: bool,
    pub intelligence_proficent: bool,
    pub wisdom_proficent: bool,
    pub charisma_proficent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Skills {
    pub acrobatics: String,
    pub animal_handling: String,
    pub arcana: String,
    pub athletics: String,
    pub deception: String,
    pub history: String,
    pub insight: String,
    pub intimidation: String,
    pub investigation: String,
    pub medicine: String,
    pub nature: String,
    pub perception: String,
    pub performance: String,
    pub persuasion: String,
    pub religion: String,
    pub slight_of_hand: String,
    pub stealth: String,
    pub survival: String,
    pub acrobatics_skill: String,
    pub animal_handling_skill: String,
    pub arcana_skill: String,
    pub athletics_skill: String,
    pub deception_skill: String,
    pub history_skill: String,
    pub insight_skill: String,
    pub intimidation_skill: String,
    pub investigation_skill: String,
    pub medicine_skill: String,
    pub nature_skill: String,
    pub perception_skill: String,
    pub performance_skill: String,
    pub persuasion_skill: String,
    pub religion_skill: String,
    pub slight_of_hand_skill: String,
    pub stealth_skill: String,
    pub survival_skill: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct ProficienciesAndLanguage {
    pub languages_known: String,
    pub armor_proficiency: String,
    pub weapon_proficiency: String,
    pub tools_proficiency: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Health {
    pub armor_class: u8,
    pub initiative: String,
    pub speed: u8,
    pub current_hp: u8,
    pub maximum_hp: u8,
    pub temporary_hp: u8,
    pub hit_dice_type: String,
    pub total_hit_dice: u8,
    pub current_hit_dice: u8,
    pub unconscious: bool,
    pub death_save_saves: String,
    pub death_save_fails: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CharSheet {
    pub information: Information,
    pub statistics: Statistics,
    pub saving_throws: SavingThrows,
    pub skills: Skills,
    pub proficiencies_and_language: ProficienciesAndLanguage,
    pub health: Health,
}

#[derive(Clone, Copy)]
pub struct StatView {
    pub name: &'static str,
    pub value: u8,
    pub modifier: i8,
}

impl Information {
    pub fn information_to_list_item(&self) -> Vec<ListItem<'static>> {
        vec![
            ListItem::new(format!("Char Name: {}", self.character_name)),
            ListItem::new(format!("Class: {}", self.class)),
            ListItem::new(format!("Level: {}", self.level)),
            ListItem::new(format!("Background: {}", self.background)),
            ListItem::new(format!("Player Name: {}", self.player_name)),
            ListItem::new(format!("Race: {}", self.race)),
            ListItem::new(format!("Alignment: {}", self.alignment)),
            ListItem::new(format!("Experience: {}", self.experience)),
        ]
    }
}

impl Statistics {
    pub fn ability_mod(&self, stat: u8) -> i8 {
        let mut modifier: i8 = (stat as i8 - 10) / 2;
        if stat < 10 && stat % 2 == 1 {
            modifier -= 1;
        }
        modifier
    }

    pub fn ability_scores(&self) -> [StatView; 6] {
        [
            StatView {
                name: "STR",
                value: self.strength,
                modifier: self.ability_mod(self.strength),
            },
            StatView {
                name: "DEX",
                value: self.dexterity,
                modifier: self.ability_mod(self.dexterity),
            },
            StatView {
                name: "CON",
                value: self.constitution,
                modifier: self.ability_mod(self.constitution),
            },
            StatView {
                name: "INT",
                value: self.intelligence,
                modifier: self.ability_mod(self.intelligence),
            },
            StatView {
                name: "WIS",
                value: self.wisdom,
                modifier: self.ability_mod(self.wisdom),
            },
            StatView {
                name: "CHA",
                value: self.charisma,
                modifier: self.ability_mod(self.charisma),
            },
        ]
    }
}

#[derive(Clone, Copy)]
pub struct SavingThrowView {
    pub name: &'static str,
    pub value: i8,
    pub proficient: bool,
}

impl SavingThrowView {
    pub fn symbol(&self) -> &'static str {
        if !self.proficient {
            "○"
        } else {
            "●"
        }
    }
}

impl SavingThrows {
    pub fn saving_throw_views(&self, stats: &Statistics) -> [SavingThrowView; 6] {
        [
            Self::saving_throw(
                "STR",
                stats.strength,
                self.strength_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "DEX",
                stats.dexterity,
                self.dexterity_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "CON",
                stats.constitution,
                self.constitution_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "INT",
                stats.intelligence,
                self.intelligence_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "WIS",
                stats.wisdom,
                self.wisdom_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "CHA",
                stats.charisma,
                self.charisma_proficent,
                stats.proficiency_bonus,
                stats,
            ),
        ]
    }

    fn saving_throw(
        name: &'static str,
        score: u8,
        proficient: bool,
        prof_bonus: u8,
        stats: &Statistics,
    ) -> SavingThrowView {
        let mut value = Statistics::ability_mod(stats, score);

        if proficient {
            value += prof_bonus as i8;
        }

        SavingThrowView {
            name,
            value,
            proficient,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SkillsView {
    pub name: &'static str,
    pub value: i8,
    pub sp: SkillProficiency,
}

impl Skills {
    pub fn skills_views(&self) -> [SkillsView; 18] {
        [
            Self::skills(
                "Acrobatics (Dex)",
                self.acrobatics.clone(),
                self.acrobatics_skill.clone(),
            ),
            Self::skills(
                "Animal Handling (Wis)",
                self.animal_handling.clone(),
                self.animal_handling_skill.clone(),
            ),
            Self::skills(
                "Arcana (Int)",
                self.arcana.clone(),
                self.arcana_skill.clone(),
            ),
            Self::skills(
                "Athletics (Str)",
                self.athletics.clone(),
                self.athletics_skill.clone(),
            ),
            Self::skills(
                "Deception (Dex)",
                self.deception.clone(),
                self.deception_skill.clone(),
            ),
            Self::skills(
                "History (Int)",
                self.history.clone(),
                self.history_skill.clone(),
            ),
            Self::skills(
                "Insight (Wis)",
                self.insight.clone(),
                self.insight_skill.clone(),
            ),
            Self::skills(
                "Intimidation (Cha)",
                self.intimidation.clone(),
                self.intimidation_skill.clone(),
            ),
            Self::skills(
                "Investigation (Int)",
                self.investigation.clone(),
                self.investigation_skill.clone(),
            ),
            Self::skills(
                "Medicine (Wis)",
                self.medicine.clone(),
                self.medicine_skill.clone(),
            ),
            Self::skills(
                "Nature (Int)",
                self.nature.clone(),
                self.nature_skill.clone(),
            ),
            Self::skills(
                "Perception (Wis)",
                self.perception.clone(),
                self.perception_skill.clone(),
            ),
            Self::skills(
                "Performance (Cha)",
                self.performance.clone(),
                self.performance_skill.clone(),
            ),
            Self::skills(
                "Persuasion (Cha)",
                self.persuasion.clone(),
                self.persuasion_skill.clone(),
            ),
            Self::skills(
                "Religion (Int)",
                self.religion.clone(),
                self.religion_skill.clone(),
            ),
            Self::skills(
                "Slight of Hand (Dex)",
                self.slight_of_hand.clone(),
                self.slight_of_hand_skill.clone(),
            ),
            Self::skills(
                "Stealth (Dex)",
                self.stealth.clone(),
                self.stealth_skill.clone(),
            ),
            Self::skills(
                "Survival (Wis)",
                self.survival.clone(),
                self.survival_skill.clone(),
            ),
        ]
    }

    fn skills(name: &'static str, score: String, proficient_str: String) -> SkillsView {
        let value;
        let mut sp: SkillProficiency = SkillProficiency::None;

        if score.is_empty() {
            value = 0;
        } else {
            let value_opt = parse_string(&score);
            match value_opt {
                Some(val) => {
                    value = val;
                }
                None => {
                    value = 0;
                }
            }
        }

        if proficient_str == "proficient" {
            sp = SkillProficiency::Proficient;
        } else if proficient_str == "expertise" {
            sp = SkillProficiency::Expertise;
        }

        SkillsView { name, value, sp }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SkillProficiency {
    None,
    Proficient,
    Expertise,
}

impl SkillProficiency {
    pub fn symbol(self) -> &'static str {
        match self {
            SkillProficiency::None => "○",
            SkillProficiency::Proficient => "●",
            SkillProficiency::Expertise => "◎",
        }
    }
}

fn parse_string(s: &str) -> Option<i8> {
    match s.parse::<i8>() {
        Ok(num) => Some(num),
        Err(e) => {
            println!("Failed to parse \"{}\": {}", s, e);
            None
        }
    }
}

impl Health {
    pub fn increase(&mut self) {
        self.current_hp = (self.current_hp + 1).min(self.maximum_hp);
    }

    pub fn decrease(&mut self) {
        if self.current_hp >= 1 {
            self.current_hp = self.current_hp - 1;
        } else {
            self.current_hp = 0;
        }
    }
}

#[derive(Default)]
pub struct ViewState {
    pub health: HealthView,
    // TODO: Move other View's into this structure
    // stats_view, skills_view, etc
}

#[derive(Default)]
pub enum Hover {
    Minus,
    Plus,
    #[default]
    None,
}

#[derive(Default)]
pub struct HealthView {
    pub minus_rect: Rect,
    pub plus_rect: Rect,
    pub hover: Hover,
}

pub enum CurrentScreen {
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

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub char_sheet: CharSheet,
    pub json_file_name: String,
}

impl App {
    pub fn new(json_file: String) -> App {
        let mut file = File::open(&json_file).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let loaded_char_sheet: CharSheet = serde_json::from_str(&buff).unwrap();
        App {
            current_screen: CurrentScreen::Main,
            char_sheet: loaded_char_sheet,
            json_file_name: json_file.clone(),
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("Closing the file and writing it disk");
        // Here you'd close a file, free memory, etc.
        // 2. Create the output file
        let file = File::create(self.json_file_name.clone()).unwrap();
        let mut writer = BufWriter::new(file); // Use BufWriter for performance
        let _ = serde_json::to_writer_pretty(&mut writer, &self.char_sheet);

        writer.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hp_increase_caps_at_max() {
        let mut h = Health {
            armor_class: 0,
            initiative: "+1".to_string(),
            speed: 30,
            current_hp: 9,
            maximum_hp: 10,
            temporary_hp: 0,
            hit_dice_type: "d12".to_string(),
            total_hit_dice: 6,
            current_hit_dice: 6,
            unconscious: false,
            death_save_saves: "".to_string(),
            death_save_fails: "".to_string(),
        };
        h.increase();
        h.increase();
        assert_eq!(h.current_hp, 10);
    }

    #[test]
    fn hp_decrease_stops_at_zero() {
        let mut h = Health {
            armor_class: 0,
            initiative: "+1".to_string(),
            speed: 30,
            current_hp: 1,
            maximum_hp: 10,
            temporary_hp: 0,
            hit_dice_type: "d12".to_string(),
            total_hit_dice: 6,
            current_hit_dice: 6,
            unconscious: false,
            death_save_saves: "".to_string(),
            death_save_fails: "".to_string(),
        };
        h.decrease();
        h.decrease();
        assert_eq!(h.current_hp, 0);
    }

    #[test]
    fn skills_proficiency_symbol_none() {
        let skill_prof: SkillProficiency = SkillProficiency::None;
        assert_eq!(skill_prof.symbol(), "○");
    }

    #[test]
    fn skills_proficiency_symbol_proficient() {
        let skill_prof: SkillProficiency = SkillProficiency::Proficient;
        assert_eq!(skill_prof.symbol(), "●");
    }

    #[test]
    fn skills_proficiency_symbol_expertise() {
        let skill_prof: SkillProficiency = SkillProficiency::Expertise;
        assert_eq!(skill_prof.symbol(), "◎");
    }

    #[test]
    fn saving_throw_view_symbol_none() {
        let stv: SavingThrowView = SavingThrowView {
            name: "test",
            value: 1,
            proficient: false,
        };
        assert_eq!(stv.symbol(), "○");
    }

    #[test]
    fn saving_throw_view_symbol_proficient() {
        let stv: SavingThrowView = SavingThrowView {
            name: "test",
            value: 1,
            proficient: true,
        };
        assert_eq!(stv.symbol(), "●");
    }
}
