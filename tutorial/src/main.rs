use rusty_engine::prelude::{
    bevy::log::{debug, info},
    *,
};

#[derive(Resource)]
struct GameState {
    // high_score: u32,
    current_score: u32,
    enemy_index: i32,
    // spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // high_score: 0,
            current_score: 0,
            enemy_index: 0,
            // spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // handle collisions
    // engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        debug!("event: {:?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // remove the sprite the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.current_score += 1;
            info!("Current score: {}", game_state.current_score);
        }
    }

    // handle movement
    const MOVEMENT_SPPED: f32 = 100.0;
    let player = engine.sprites.get_mut("player").unwrap();
    // Up
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowUp, KeyCode::KeyW])
    {
        player.translation.y += MOVEMENT_SPPED * engine.delta_f32;
    }
    // Down
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowDown, KeyCode::KeyS])
    {
        player.translation.y -= MOVEMENT_SPPED * engine.delta_f32;
    }
    // Right
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowLeft, KeyCode::KeyA])
    {
        player.translation.x -= MOVEMENT_SPPED * engine.delta_f32;
    }
    // Down
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowRight, KeyCode::KeyD])
    {
        player.translation.x += MOVEMENT_SPPED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("car{}", game_state.enemy_index);
            game_state.enemy_index += 1;
            let car = engine.add_sprite(label, SpritePreset::RacingCarYellow);
            car.translation = mouse_location;
            car.collision = true;
        }
    }
}

fn main() {
    let mut game = Game::new();

    // setup game here
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}
