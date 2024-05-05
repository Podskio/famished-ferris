use crate::{
    game::{environment_at_position, GameState},
    GAME_SIZE,
};
use crossterm::event::KeyCode;

pub fn handle_game_input(state: &mut GameState, key: KeyCode) {
    match key {
        KeyCode::Char('w') | KeyCode::Up => {
            if state.player.position.1 > 0
                && !environment_at_position(
                    (state.player.position.0, state.player.position.1 - 1),
                    state,
                )
            {
                state.player.position.1 -= 1;
            }
        }
        KeyCode::Char('s') | KeyCode::Down => {
            if state.player.position.1 < GAME_SIZE - 1
                && !environment_at_position(
                    (state.player.position.0, state.player.position.1 + 1),
                    state,
                )
            {
                state.player.position.1 += 1;
            }
        }
        KeyCode::Char('a') | KeyCode::Left => {
            if state.player.position.0 > 0
                && !environment_at_position(
                    (state.player.position.0 - 1, state.player.position.1),
                    state,
                )
            {
                state.player.position.0 -= 1;
            }
        }
        KeyCode::Char('d') | KeyCode::Right => {
            if state.player.position.0 < GAME_SIZE - 1
                && !environment_at_position(
                    (state.player.position.0 + 1, state.player.position.1),
                    state,
                )
            {
                state.player.position.0 += 1;
            }
        }
        _ => {}
    }
}
