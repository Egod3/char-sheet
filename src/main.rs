use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Information {
    character_name: String,
    class: String,
    level: u8,
    background: String,
    player_name: String,
    race: String,
    alignment: String,
    experience: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
// Modifiers will be calculated based on rules of the game
struct Statistics {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charasima: u8,
    inspiration: bool,
    proficiency_bonus: u8,
    passive_wisdom_perception: u8,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SavingThrows {
    strength_proficent: bool,
    dexterity_proficent: bool,
    constitution_proficent: bool,
    intelligence_proficent: bool,
    wisdom_proficent: bool,
    charasima_proficent: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Skills {
    acrobatics: String,
    animal_handling: String,
    arcana: String,
    athletics: String,
    deception: String,
    history: String,
    insight: String,
    investigation: String,
    medicine: String,
    nature: String,
    perception: String,
    performance: String,
    persuasion: String,
    religion: String,
    slight_of_hand: String,
    stealth: String,
    survival: String,
    acrobatics_skill: String,
    animal_handling_skill: String,
    arcana_skill: String,
    athletics_skill: String,
    deception_skill: String,
    history_skill: String,
    insight_skill: String,
    investigation_skill: String,
    medicine_skill: String,
    nature_skill: String,
    perception_skill: String,
    performance_skill: String,
    persuasion_skill: String,
    religion_skill: String,
    slight_of_hand_skill: String,
    stealth_skill: String,
    survival_skill: String,
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ProficienciesAndLanguage {
    languages_known: String,
    armor_proficiency: String,
    weapon_proficiency: String,
    tools_proficiency: String,
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Health {
    armor_class: u8,
    initiative: String,
    speed: u8,
    current_hp: u8,
    temporary_hp: u8,
    hit_dice_type: String,
    total_hit_dice: u8,
    current_hit_dice: u8,
    unconcicious: bool,
    death_save_saves: String,
    death_save_failes: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CharSheet {
    information: Information,
    statistics: Statistics,
    saving_throws: SavingThrows,
    skills: Skills,
    proficiencies_and_language: ProficienciesAndLanguage,
    health: Health,
}

fn main() -> Result<()> {
    println!("welcome to char-sheet");

    // Exmple sheet that is based on DND 5e Cromwell Windscream a Path of the Giant Barbarian
    let mut file = File::open("resources/character_sheet.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let char_sheet: CharSheet = serde_json::from_str(&buff).unwrap();
    let information: Information = char_sheet.information;
    let statistics: Statistics = char_sheet.statistics;
    let saving_throws: SavingThrows = char_sheet.saving_throws;
    let skills: Skills = char_sheet.skills;
    let proficiencies_and_language: ProficienciesAndLanguage =
        char_sheet.proficiencies_and_language;
    let health: Health = char_sheet.health;

    println!("information.character_name: {}", information.character_name);
    println!("information.class: {}", information.class);
    println!("information.level: {}", information.level);
    println!("information.background: {}", information.background);
    println!("information.player_name: {}", information.player_name);
    println!("information.race: {}", information.race);
    println!("information.alignment: {}", information.alignment);
    println!("information.experience: {}", information.experience);

    println!("statistics.strength: {}", statistics.strength);

    println!(
        "saving_throws.strength_proficent: {}",
        saving_throws.strength_proficent
    );

    println!("skills.acrobatics: {}", skills.acrobatics);

    println!(
        "proficiencies_and_language.languages_known: {}",
        proficiencies_and_language.languages_known
    );

    println!("health.armor_class: {}", health.armor_class);

    Ok(())
}
