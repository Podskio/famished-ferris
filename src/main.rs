use crossterm::event::{self, KeyCode, KeyEventKind};
use game::{
    generate_objects, get_num_food, get_object_in_position, handle_object_collision, EndReason,
    GameState,
};
use interface::{
    render_end_screen, render_game_board, render_score_board, render_start_screen, render_title,
};
use keyboard::handle_game_input;
use terminal::cleanup_terminal;

mod game;
mod interface;
mod keyboard;
mod terminal;

// Must be an odd number so the player can spawn in the center
pub const GAME_SIZE: u16 = 23;

// Start by decreasing hunger every 5 seconds
const INITIAL_HUNGER_RATE: u16 = 5 * 10;

fn main() {
    let mut terminal = terminal::setup_terminal();

    let mut state = GameState::default();
    state.hunger_rate = INITIAL_HUNGER_RATE;
    generate_objects(&mut state);

    // Start screen
    loop {
        terminal
            .draw(|frame| {
                render_title(frame);
                render_start_screen(frame);
            })
            .unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
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

                if get_num_food(&state) == 0 {
                    state.end_reason = Some(EndReason::Won);
                }
            })
            .unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Char('q') {
                    cleanup_terminal();
                    return;
                }

                handle_game_input(&mut state, key.code);

                match get_object_in_position(state.player.position, &state) {
                    Some(collision) => handle_object_collision(collision, &mut state),
                    None => {}
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

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    cleanup_terminal();
}
