use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{
    App, CurrentScreen, HealthView, Hover, SavingThrowView, SkillsView, StatView, ViewState,
};

use std::rc::Rc;

fn render_stat(frame: &mut Frame, stat: StatView, area: ratatui::layout::Rect) {
    let lines = vec![Line::from(format!("{:+} ({:})", stat.modifier, stat.value))];

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(stat.name))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}

fn render_saving_throw(frame: &mut Frame, st: SavingThrowView, area: Rect) {
    let symbol = if st.proficient { "●" } else { "○" };

    let value_style = if st.value >= 0 {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Red)
    };

    let line = Line::from(vec![
        Span::raw(format!("{} ", symbol)),
        Span::raw(format!("{:<3} ", st.name)),
        Span::styled(format!("{:+}", st.value), value_style),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}

fn skill_to_list_item(skill: &SkillsView) -> ListItem<'static> {
    ListItem::new(format!(
        "{} {:<14} {:+}",
        skill.sp.symbol(),
        skill.name,
        skill.value,
    ))
}

fn draw_char_info(frame: &mut Frame, area: Rect, app: &App) {
    let info_blk = Block::default()
        .borders(Borders::ALL)
        .title("Character Information")
        .style(Style::default().fg(Color::Green));
    frame.render_widget(info_blk.clone(), area);

    let inner_info_frame = info_blk.inner(area);
    let char_info_width = 40;
    // Split the "Character info" area into 3 rows;
    let char_info_rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(char_info_width + 10), // slot 0
            Constraint::Length(char_info_width - 10), // slot 1
            Constraint::Length(char_info_width - 10), // slot 2
        ])
        .split(inner_info_frame);
    let info_list = app.char_sheet.information.information_to_list_item();
    let char_info_row_len = 3;

    let char_info_items_zero: Vec<ListItem> = info_list[0..char_info_row_len].to_vec();
    let char_info_items_one: Vec<ListItem> =
        info_list[char_info_row_len..char_info_row_len * 2].to_vec();
    let end = 8;
    let char_info_items_two: Vec<ListItem> = info_list[char_info_row_len * 2..end].to_vec();

    frame.render_widget(List::new(char_info_items_zero), char_info_rows[0]);
    frame.render_widget(List::new(char_info_items_one), char_info_rows[1]);
    frame.render_widget(List::new(char_info_items_two), char_info_rows[2]);
}

fn draw_abilities(frame: &mut Frame, area: Rect, app: &App) {
    let stats_blk = Block::default()
        .borders(Borders::ALL)
        .title("Abilities")
        .style(Style::default().fg(Color::Green));
    frame.render_widget(stats_blk.clone(), area);

    let stats_sv = app.char_sheet.statistics.ability_scores();
    let inner_stats_frame = stats_blk.inner(area);
    // Split the "Abilities" area into 3;
    // left side: statistics, middle: saving throws, right side: skills
    let ability_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Statistics grid
            Constraint::Length(16), // Savings throw box
            Constraint::Length(58), // Skills box
        ])
        .split(inner_stats_frame);

    let stat_box_height = 3;
    let stats_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(stat_box_height),
            Constraint::Length(stat_box_height),
            Constraint::Length(stat_box_height),
            Constraint::Min(0),
        ])
        .split(ability_chunks[0]);

    let stats_row1 = stats_rows[1];
    let stats_row2 = stats_rows[2];
    let stats_row3 = stats_rows[3];

    let stat_width = 12;

    let row1_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(stat_width),
            Constraint::Length(stat_width),
        ])
        .split(stats_row1);

    let row2_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(stat_width),
            Constraint::Length(stat_width),
        ])
        .split(stats_row2);

    let row3_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(stat_width),
            Constraint::Length(stat_width),
        ])
        .split(stats_row3);

    // Render the stats 2 per row and modifiers here:
    for (stat, chunk) in stats_sv.into_iter().take(2).zip(row1_chunks.iter()) {
        render_stat(frame, stat, *chunk);
    }

    // Render the stats 2 per row and modifiers here:
    for (stat, chunk) in stats_sv.into_iter().skip(2).zip(row2_chunks.iter()) {
        render_stat(frame, stat, *chunk);
    }

    // Render the stats 2 per row and modifiers here:
    for (stat, chunk) in stats_sv.into_iter().skip(4).zip(row3_chunks.iter()) {
        render_stat(frame, stat, *chunk);
    }

    let saving_throws = app
        .char_sheet
        .saving_throws
        .saving_throw_views(&app.char_sheet.statistics);

    let sav_thr_blk = Block::default()
        .borders(Borders::ALL)
        .title("Saving Throws");

    let sav_thr_inner = sav_thr_blk.inner(ability_chunks[1]);

    let svn_thr_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(8); saving_throws.len()])
        .split(sav_thr_inner);

    frame.render_widget(sav_thr_blk, ability_chunks[1]);

    for (st, row) in saving_throws.into_iter().zip(svn_thr_rows.iter()) {
        render_saving_throw(frame, st, *row);
    }

    let skills_blk = Block::default().borders(Borders::ALL).title("Skills");

    let skills_inner = skills_blk.inner(ability_chunks[2]);

    frame.render_widget(&skills_blk, ability_chunks[2]);

    // Get the skills_view array
    let skills_views = app.char_sheet.skills.skills_views();
    let skills_row_size = skills_views.len() / 2;
    let skills_box_width = 28;
    let skills_rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(skills_box_width),
            Constraint::Length(skills_box_width),
        ])
        .split(skills_inner);

    let skills_items_zero: Vec<ListItem> = skills_views
        .iter()
        .take(skills_row_size)
        .map(skill_to_list_item)
        .collect();
    let skills_items_one: Vec<ListItem> = skills_views
        .iter()
        .skip(skills_row_size)
        .map(skill_to_list_item)
        .collect();

    frame.render_widget(List::new(skills_items_zero), skills_rows[0]);
    frame.render_widget(List::new(skills_items_one), skills_rows[1]);
}

fn draw_health(frame: &mut Frame, area: Rect, app: &App, view: &mut HealthView) {
    let health_blk = Block::default()
        .borders(Borders::ALL)
        .title("Health")
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(health_blk.clone(), area);

    let inner_health_frame = health_blk.inner(area);
    let health_width = 1;
    let health_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(health_width),
            Constraint::Length(health_width),
            Constraint::Length(health_width),
        ])
        .split(inner_health_frame);

    let hp_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(2, 3), // current HP (larger)
            Constraint::Ratio(1, 3), // temp HP (smaller)
        ])
        .split(health_rows[0]);

    let current_hp = Paragraph::new(Line::from(vec![
        Span::raw("Current: "),
        Span::styled(
            format!(
                "{}/{}",
                app.char_sheet.health.current_hp, app.char_sheet.health.maximum_hp
            ),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));

    frame.render_widget(current_hp, hp_row[0]);

    let mut temp_text: String = "--".into();
    if app.char_sheet.health.temporary_hp != 0 {
        temp_text = app.char_sheet.health.temporary_hp.to_string();
    }

    let temp_hp = Paragraph::new(Line::from(vec![
        Span::raw("Temp: "),
        Span::styled(temp_text, Style::default().add_modifier(Modifier::DIM)),
    ]))
    .alignment(Alignment::Right);

    frame.render_widget(temp_hp, hp_row[1]);

    let health_controls = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(6), // [-]
            Constraint::Min(1),    // label
            Constraint::Length(6), // [+]
        ])
        .split(health_rows[2]);

    view.minus_rect = health_controls[0];
    view.plus_rect = health_controls[2];

    let minus_style = if matches!(view.hover, Hover::Minus) {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::REVERSED)
    } else {
        Style::default().add_modifier(Modifier::REVERSED)
    };

    let plus_style = if matches!(view.hover, Hover::Plus) {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::REVERSED)
    } else {
        Style::default().add_modifier(Modifier::REVERSED)
    };

    frame.render_widget(
        Paragraph::new(" [ - ] ")
            .alignment(Alignment::Center)
            .style(minus_style),
        health_controls[0],
    );

    // Render the label
    frame.render_widget(
        Paragraph::new("Adjust HP").alignment(Alignment::Center),
        health_controls[1],
    );

    frame.render_widget(
        Paragraph::new(" [ + ] ")
            .alignment(Alignment::Center)
            .style(plus_style),
        health_controls[2],
    );
}

pub fn draw_title(frame: &mut Frame) -> Rc<[Rect]> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title Header                          0
            Constraint::Max(5),    // Information                           1
            Constraint::Max(5),    // Health                                2
            Constraint::Max(13),   // Statistics, Saving_throws & Skills    3
            Constraint::Min(5),    // Prof and Language                     4
            Constraint::Length(3), // Footer                                5
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Create paragraph for base app
    let title = Paragraph::new(Text::styled(
        "D&D Character Sheet",
        Style::default().fg(Color::Green),
    ))
    .block(title_block.clone());

    frame.render_widget(title, chunks[0]);

    chunks
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    /*
     * Here, we will create a Vec of Span which will be converted later into
     * a single line by the Paragraph. (A Span is different from a Line,
     * because a Span indicates a section of Text with a style applied,
     * and doesn’t end with a newline)
     */
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("View Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
            CurrentScreen::Exiting => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

pub fn ui(frame: &mut Frame, app: &mut App, view_state: &mut ViewState) {
    let chunks = draw_title(frame);

    let info_chunk = chunks[1];
    let health_chunk = chunks[2];
    let stats_chunk = chunks[3];
    let _prof_and_lang_chunk = chunks[4];
    let footer_chunk = chunks[chunks.len() - 1];

    draw_char_info(frame, info_chunk, app);

    draw_abilities(frame, stats_chunk, app);

    // Create a Rectangle to display player AC/HP/Temp HP/Initiative/Speed
    // I am thinking of having the death saves/death fails
    // be hidden until the player goes to 0 HP then have that pop up.
    // I need to think through the use cases and ensure we support
    // the char dying and being revived and being "down" but able to try death saves.
    draw_health(frame, health_chunk, app, &mut view_state.health);

    draw_footer(frame, footer_chunk, app);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
