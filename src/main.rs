use crossterm::event::{self, KeyCode, KeyEventKind};
use game::{
    generate_objects, get_object_in_position, handle_object_collision, EndReason, GameState,
};
use interface::{
    render_end_screen, render_game_board, render_score_board, render_start_screen, render_title,
};
use keyboard::handle_game_input;
use ratatui::{
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};
use std::{thread::sleep, time::Duration};
use terminal::{cleanup_terminal, setup_terminal};

mod game;
mod interface;
mod keyboard;
mod terminal;

// Must be an odd number so the player can spawn in the center
pub const GAME_SIZE: u16 = 23;

// Start by decreasing hunger every 5 seconds
const INITIAL_HUNGER_RATE: u16 = 5 * 10;

fn main() {
    let mut terminal = setup_terminal();

    // Wait until terminal is large enough
    while terminal.size().unwrap().height < (GAME_SIZE + 17) {
        terminal
            .draw(|frame| {
                frame.render_widget(
                    Paragraph::new("The terminal is not large enough to render the game.\n\nPlease increase the window height of the terminal or decrease the font size.").light_yellow()
                        .block(Block::default().borders(Borders::ALL))
                        .centered(),
                    frame.size(),
                );
            })
            .unwrap();

        sleep(Duration::from_millis(100));
    }

    let mut state = GameState {
        hunger_rate: INITIAL_HUNGER_RATE,
        ..GameState::default()
    };

    generate_objects(&mut state);

    // Start screen
    loop {
        terminal
            .draw(|frame| {
                render_title(frame);
                render_start_screen(frame);
            })
            .unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                    break;
                }
            }
        }
    }

    // Main game loop
    while state.end_reason.is_none() {
        state.time += 1;

        // Decrease hunger by 1 every 5 seconds
        if state.time % state.hunger_rate == 0 {
            state.player.hunger -= 1;
        }

        terminal
            .draw(|frame| {
                render_title(frame);
                render_game_board(&state, frame);
                render_score_board(&state, frame);

                if state.player.hunger == 0 {
                    state.end_reason = Some(EndReason::Starved);
                }

                if state.player.health == 0 {
                    state.end_reason = Some(EndReason::Died);
                }
            })
            .unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Char('q') {
                    cleanup_terminal();
                    return;
                }

                handle_game_input(&mut state, key.code);

                if let Some(collision) = get_object_in_position(state.player.position, &state) {
                    handle_object_collision(collision, &mut state);
                }
            }
        }
    }

    // End screen
    loop {
        terminal
            .draw(|frame| {
                render_title(frame);
                render_end_screen(&state, frame);
            })
            .unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    cleanup_terminal();
}
