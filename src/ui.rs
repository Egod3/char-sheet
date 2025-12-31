use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing, SavingThrowView, SkillsView, StatView};

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

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title Header
            Constraint::Max(10),   // info
            Constraint::Max(13),   // statistics, saving_throws & skills
            //Constraint::Min(5),    // prof and language
            Constraint::Min(5),    // health
            Constraint::Length(3), // Footer
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

    let title_chunk = chunks[0];
    let info_chunk = chunks[1];
    let stats_chunk = chunks[2];
    let footer_chunk = chunks[chunks.len() - 1];
    frame.render_widget(title, title_chunk);

    let info_paragraph = Paragraph::new(app.char_sheet.information.get_info_text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Character Information"),
        )
        .style(Style::default().fg(Color::Green));
    frame.render_widget(info_paragraph, info_chunk);

    let stats_blk = Block::default().borders(Borders::ALL).title("Abilities");
    frame.render_widget(stats_blk.clone(), stats_chunk);
    let inner_stats_frame = stats_blk.inner(stats_chunk);

    let stats_sv = app.char_sheet.statistics.ability_scores();
    // Split the area into 2; left side: statistics right side: saving throws
    let stats_chunks = Layout::default()
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
        .split(stats_chunks[0]);

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

    let sav_thr_inner = sav_thr_blk.inner(stats_chunks[1]);

    let svn_thr_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(8); saving_throws.len()])
        .split(sav_thr_inner);

    frame.render_widget(sav_thr_blk, stats_chunks[1]);

    for (st, row) in saving_throws.into_iter().zip(svn_thr_rows.iter()) {
        render_saving_throw(frame, st, *row);
    }

    let skills_blk = Block::default().borders(Borders::ALL).title("Skills");

    let skills_inner = skills_blk.inner(stats_chunks[2]);

    frame.render_widget(&skills_blk, stats_chunks[2]);

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
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(footer_chunk);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }

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
