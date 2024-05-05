use crate::GAME_SIZE;

const MAX_HEALTH: u8 = 5;
const MAX_HUNGER: u8 = 10;

pub const FOOD_EMOJIS: [&str; 2] = ["🦐", "🪱"];
pub const PREDATOR_EMOJIS: [&str; 5] = ["🪼", "🐙", "🦈", "🐢", "🚣"];
pub const ENVIRONMENT_EMOJIS: [&str; 2] = ["🪸", "🐚"];

const OBJECT_VARIANTS: &[(ObjectVariant, f32, &[&str])] = &[
    (ObjectVariant::Food, 0.05, FOOD_EMOJIS.as_slice()),
    (ObjectVariant::Predator, 0.025, PREDATOR_EMOJIS.as_slice()),
    (
        ObjectVariant::Environment,
        0.1,
        ENVIRONMENT_EMOJIS.as_slice(),
    ),
];

#[derive(PartialEq, Clone, Copy)]
pub enum ObjectVariant {
    Food,
    Predator,
    Environment,
}

pub struct Object {
    pub variant: ObjectVariant,
    pub position: (u16, u16),
    pub emoji: &'static str,
}

pub struct Player {
    pub position: (u16, u16),
    pub health: u8,
    pub hunger: u8,
}

pub enum EndReason {
    Starved,
    Died,
    Won,
}

#[derive(Default)]
pub struct GameState {
    pub player: Player,
    pub time: u32,
    pub objects: Vec<Object>,
    pub end_reason: Option<EndReason>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: (GAME_SIZE / 2, GAME_SIZE / 2),
            health: MAX_HEALTH,
            hunger: MAX_HUNGER,
        }
    }
}

fn get_empty_position(state: &GameState) -> (u16, u16) {
    let mut position = (fastrand::u16(0..GAME_SIZE), fastrand::u16(0..GAME_SIZE));

    while get_object_in_position(position, state).is_some() || state.player.position == position {
        position = (fastrand::u16(0..GAME_SIZE), fastrand::u16(0..GAME_SIZE));
    }

    position
}

pub fn generate_objects(state: &mut GameState) {
    let num_positions = GAME_SIZE * GAME_SIZE;

    for (variant, rate, emojis) in OBJECT_VARIANTS {
        let num_objects = num_positions as f32 * rate;

        for _ in 0..num_objects as u16 {
            state.objects.push(Object {
                variant: *variant,
                position: get_empty_position(state),
                emoji: fastrand::choice(*emojis).unwrap(),
            });
        }
    }
}

pub fn get_object_in_position(
    position: (u16, u16),
    state: &GameState,
) -> Option<(usize, ObjectVariant)> {
    let index = state
        .objects
        .iter()
        .position(|obj| obj.position == position)?;

    let object = state.objects.iter().find(|obj| obj.position == position)?;

    Some((index, object.variant))
}

pub fn get_num_food(state: &GameState) -> usize {
    state
        .objects
        .iter()
        .filter(|obj| obj.variant == ObjectVariant::Food)
        .count()
}

pub fn environment_at_position(position: (u16, u16), state: &GameState) -> bool {
    state
        .objects
        .iter()
        .any(|obj| obj.position == position && obj.variant == ObjectVariant::Environment)
}

pub fn handle_object_collision((i, variant): (usize, ObjectVariant), state: &mut GameState) {
    match variant {
        ObjectVariant::Food => {
            state.player.hunger = (state.player.hunger + 1).min(MAX_HUNGER);
            state.objects.remove(i);

            state.objects.push(Object {
                variant: ObjectVariant::Food,
                position: get_empty_position(state),
                emoji: fastrand::choice(FOOD_EMOJIS).unwrap(),
            });
        }
        ObjectVariant::Predator => {
            state.player.health = (state.player.health - 1).max(0);
        }
        ObjectVariant::Environment => {}
    }
}
