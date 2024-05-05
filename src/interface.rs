use crate::{
    game::{GameState, ENVIRONMENT_EMOJIS, FOOD_EMOJIS, PREDATOR_EMOJIS, TREASURE_EMOJIS},
    GAME_SIZE,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn render_title(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(
            "░█▀▀░█▀█░█▄█░▀█▀░█▀▀░█░█░█▀▀░█▀▄░░░█▀▀░█▀▀░█▀▄░█▀▄░▀█▀░█▀▀\n░█▀▀░█▀█░█░█░░█░░▀▀█░█▀█░█▀▀░█░█░░░█▀▀░█▀▀░█▀▄░█▀▄░░█░░▀▀█\n░▀░░░▀░▀░▀░▀░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀▀░░░░▀░░░▀▀▀░▀░▀░▀░▀░▀▀▀░▀▀▀",
        )
        .light_red()
        .centered(),
        Rect::new(frame.size().width / 2 - 40, 2, 80, 20),
    );
}

pub fn render_start_screen(frame: &mut Frame) {
    let vert_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(7),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(frame.size());

    let hor_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
        .split(vert_layout[1]);

    frame.render_widget(
        Paragraph::new("Welcome to Famished Ferris! 🦀\n\nThe goal of the game is to...")
            .white()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1))
                    .title(" Gameplay ")
                    .title_alignment(Alignment::Center)
                    .border_style(Style::new().yellow()),
            ),
        hor_layout[1],
    );

    frame.render_widget(
        Paragraph::new(format!(
            "Food\n{}\n\nPredators\n{}\n\nTreasure\n{}\n\nEnvironment\n{}",
            FOOD_EMOJIS.join(" "),
            PREDATOR_EMOJIS.join(" "),
            TREASURE_EMOJIS.join(" "),
            ENVIRONMENT_EMOJIS.join(" ")
        ))
        .white()
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::proportional(1))
                .title(" Objects ")
                .title_alignment(Alignment::Center)
                .border_style(Style::new().green()),
        ),
        hor_layout[2],
    );

    frame.render_widget(
        Paragraph::new(
            "W / ↑: Move up\nS / ↓: Move down\nA / ←: Move left\nD / →: Move right\n\nQ: Quit game",
        )
        .white()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::proportional(1))
                .title(" Controls ")
                .title_alignment(Alignment::Center)
                .border_style(Style::new().magenta()),
        ),
        hor_layout[3],
    );

    frame.render_widget(
        Paragraph::new("\n→ Press ENTER to begin ←")
            .centered()
            .rapid_blink()
            .gray(),
        vert_layout[2],
    );
}

pub fn render_game_board(state: &GameState, frame: &mut Frame) {
    let mut game_board = vec![vec!["🟫"; GAME_SIZE as usize]; GAME_SIZE as usize];

    for obj in state.objects.iter() {
        game_board[obj.position.1 as usize][obj.position.0 as usize] = &obj.emoji;
    }

    game_board[state.player.position.1 as usize][state.player.position.0 as usize] = "🦀";

    let game_board_str = game_board
        .iter()
        .map(|row| row.join("") + "\n")
        .collect::<String>();

    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .border_style(Style::new().light_red()),
        Rect::new(
            frame.size().width / 2 - GAME_SIZE - 4,
            7,
            (GAME_SIZE * 2) + 8,
            GAME_SIZE + 4,
        ),
    );

    frame.render_widget(
        Paragraph::new(game_board_str)
            .centered()
            .bg(Color::Rgb(165, 105, 83)),
        Rect::new(
            frame.size().width / 2 - GAME_SIZE,
            9,
            GAME_SIZE * 2,
            GAME_SIZE,
        ),
    );
}

fn get_hunger_bar(hunger: u8) -> String {
    match hunger {
        1..=2 => "🟥".repeat(hunger as usize),
        3..=4 => "🟨".repeat(hunger as usize),
        _ => "🟩".repeat(hunger as usize),
    }
}

pub fn render_score_board(state: &GameState, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "💰 Treasure: {}\n💖 Health: {}\n😋 Hunger: {}",
            state.treasure,
            "❤️".repeat(state.player.health as usize),
            get_hunger_bar(state.player.hunger),
        )),
        Rect::new(
            frame.size().width / 2 - 15,
            GAME_SIZE + 12,
            frame.size().width,
            frame.size().height,
        ),
    );

    frame.render_widget(
        Paragraph::new("(press q to quit)").centered().dark_gray(),
        Rect::new(
            0,
            frame.size().height - 2,
            frame.size().width,
            frame.size().height,
        ),
    );
}

pub fn render_end_screen(_state: &GameState, _frame: &mut Frame) {
    todo!();
}
