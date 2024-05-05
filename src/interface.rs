use crate::{
    game::{EndReason, GameState, ENVIRONMENT_EMOJIS, FOOD_EMOJIS, PREDATOR_EMOJIS},
    GAME_SIZE,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
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
            Constraint::Percentage(50),
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
        Paragraph::new("Welcome to Famished Ferris! 🦀\n\nThe goal of the game is to survive for as long as possible.\n\n🔸 Collect food\n🔸 Avoid predators\n🔸 Navigate around the environment\n\nThe game ends when you run out of health or your hunger bar completely depletes. Your hunger will start decreasing slowly, but speed up with the amount of food you eat.")
            .wrap(Wrap {trim: false})
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
            "Player (you)\n🦀\n\nFood\n{}\n\nPredators\n{}\n\nEnvironment\n{}",
            FOOD_EMOJIS.join(" "),
            PREDATOR_EMOJIS.join(" "),
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
    let mut game_board = vec![vec!["🟦"; GAME_SIZE as usize]; GAME_SIZE as usize];

    for obj in state.objects.iter() {
        game_board[obj.position.1 as usize][obj.position.0 as usize] = obj.emoji;
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
            .bg(Color::Rgb(0, 166, 237)),
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
            "💖 Health: {}\n😋 Hunger: {}\n⌛ Time: {:01}:{:02}",
            "❤️".repeat(state.player.health as usize),
            get_hunger_bar(state.player.hunger),
            state.time / 600,
            state.time / 10 % 60
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
            frame.size().height - 1,
            frame.size().width,
            frame.size().height,
        ),
    );
}

pub fn render_end_screen(state: &GameState, frame: &mut Frame) {
    let vert_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(7),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(frame.size());

    let hor_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(vert_layout[1]);

    let end_reason_string = match state.end_reason.as_ref().unwrap() {
        EndReason::Died => "You ran into too many predators!",
        EndReason::Starved => "You got too hungry and passed away!",
    };

    frame.render_widget(
        Paragraph::new(format!(
            "{}\n\nYou survived for {:01}:{:02}\n\nBetter luck next time!",
            end_reason_string,
            state.time / 600,
            state.time / 10 % 60
        ))
        .centered()
        .white()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::proportional(1))
                .title(" Game Over ")
                .title_alignment(Alignment::Center)
                .border_style(Style::new().light_red()),
        ),
        hor_layout[1],
    );

    frame.render_widget(
        Paragraph::new("(press q to quit)").centered().dark_gray(),
        vert_layout[2],
    );
}
