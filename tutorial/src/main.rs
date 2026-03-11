use rand::{prelude::*, rng};
use rusty_engine::prelude::{
    bevy::log::{debug, info},
    *,
};

#[derive(Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    enemy_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            enemy_index: 0,
            spawn_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
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
            engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.6);

            game_state.score += 1;
            // info!("Current score: {}", game_state.score);
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score)
            }
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

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("car{}", game_state.enemy_index);
        game_state.enemy_index += 1;
        let car = engine.add_sprite(label, SpritePreset::RacingCarYellow);
        car.translation.x = rng().random_range(-550.0..550.0);
        car.translation.y = rng().random_range(-325.0..325.0);
        car.collision = true;
    } else {
        let spawn_timer = engine.texts.get_mut("spawn_timer").unwrap();
        spawn_timer.value = format!(
            "Spawn in {}",
            game_state.spawn_timer.remaining_secs() as i32
        );
    }

    // Reset score
    if engine.keyboard_state.just_pressed(KeyCode::KeyR) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();

        // Reset Timer
        game_state.spawn_timer.reset();
        let spawn_timer = engine.texts.get_mut("spawn_timer").unwrap();
        spawn_timer.value = format!(
            "Spawn in {}",
            game_state.spawn_timer.remaining_secs() as i32
        );
    }
}

fn main() {
    let mut game = Game::new();

    let mut window = Window::default();
    window.resolution = WindowResolution::new(1280, 720);
    game.window_settings(window);
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.3);

    // setup game here
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    let spawn_timer = game.add_text("spawn_timer", "Spawn in 5");
    spawn_timer.translation = Vec2::new(0.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}
