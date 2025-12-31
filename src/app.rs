use ratatui::text::Line;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SavingThrows {
    pub strength_proficent: bool,
    pub dexterity_proficent: bool,
    pub constitution_proficent: bool,
    pub intelligence_proficent: bool,
    pub wisdom_proficent: bool,
    pub charisma_proficent: bool,
}

#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ProficienciesAndLanguage {
    pub languages_known: String,
    pub armor_proficiency: String,
    pub weapon_proficiency: String,
    pub tools_proficiency: String,
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Health {
    pub armor_class: u8,
    pub initiative: String,
    pub speed: u8,
    pub current_hp: u8,
    pub temporary_hp: u8,
    pub hit_dice_type: String,
    pub total_hit_dice: u8,
    pub current_hit_dice: u8,
    pub unconcicious: bool,
    pub death_save_saves: String,
    pub death_save_failes: String,
}

#[derive(Debug, Deserialize)]
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

fn ability_mod(stat: u8) -> i8 {
    (stat as i8 - 10) / 2
}

impl Information {
    pub fn get_info_text(&self) -> Vec<Line<'_>> {
        // Define text for the Character Information sections
        let information_text = vec![
            Line::from(format!("Character Name: {}\n", self.character_name)),
            Line::from(format!("Class: {}\n", self.class)),
            Line::from(format!("Level: {}\n", self.level)),
            Line::from(format!("Background: {}\n", self.background)),
            Line::from(format!("Player Name: {}\n", self.player_name)),
            Line::from(format!("Race: {}\n", self.race)),
            Line::from(format!("Alignment: {}\n", self.alignment)),
            Line::from(format!("Experience: {}\n", self.experience)),
        ];
        information_text
    }
}

impl Statistics {
    pub fn ability_scores(&self) -> [StatView; 6] {
        [
            StatView {
                name: "STR",
                value: self.strength,
                modifier: ability_mod(self.strength),
            },
            StatView {
                name: "DEX",
                value: self.dexterity,
                modifier: ability_mod(self.dexterity),
            },
            StatView {
                name: "CON",
                value: self.constitution,
                modifier: ability_mod(self.constitution),
            },
            StatView {
                name: "INT",
                value: self.intelligence,
                modifier: ability_mod(self.intelligence),
            },
            StatView {
                name: "WIS",
                value: self.wisdom,
                modifier: ability_mod(self.wisdom),
            },
            StatView {
                name: "CHA",
                value: self.charisma,
                modifier: ability_mod(self.charisma),
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

impl SavingThrows {
    pub fn saving_throw_views(&self, stats: &Statistics) -> [SavingThrowView; 6] {
        [
            Self::saving_throw(
                "STR",
                stats.strength,
                self.strength_proficent,
                stats.proficiency_bonus,
            ),
            Self::saving_throw(
                "DEX",
                stats.dexterity,
                self.dexterity_proficent,
                stats.proficiency_bonus,
            ),
            Self::saving_throw(
                "CON",
                stats.constitution,
                self.constitution_proficent,
                stats.proficiency_bonus,
            ),
            Self::saving_throw(
                "INT",
                stats.intelligence,
                self.intelligence_proficent,
                stats.proficiency_bonus,
            ),
            Self::saving_throw(
                "WIS",
                stats.wisdom,
                self.wisdom_proficent,
                stats.proficiency_bonus,
            ),
            Self::saving_throw(
                "CHA",
                stats.charisma,
                self.charisma_proficent,
                stats.proficiency_bonus,
            ),
        ]
    }

    fn saving_throw(
        name: &'static str,
        score: u8,
        proficient: bool,
        prof_bonus: u8,
    ) -> SavingThrowView {
        let mut value = ability_mod(score);

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
                "Animal_handling (Wis)",
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
                "Slight_of_hand (Dex)",
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

        if score == "" {
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

#[derive(Clone, Copy)]
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

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
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
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub char_sheet: CharSheet,
}

impl App {
    pub fn new(json_file: String) -> App {
        let mut file = File::open(json_file).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let loaded_char_sheet: CharSheet = serde_json::from_str(&buff).unwrap();
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            char_sheet: loaded_char_sheet,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
}
