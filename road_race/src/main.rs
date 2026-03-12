// in src/main.rs
use rand::{prelude::*, rng};
use rusty_engine::prelude::*;

// Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
#[derive(Resource)]
struct GameState {
    health: i32,
    lost: bool,
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

// This function will be run once each frame.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    // Move road object
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }

        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = rng().random_range(800.0..1600.0);
                sprite.translation.y = rng().random_range(-300.0..300.0);
            }
        }
    }

    // Collect keyboard input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    // Move the player sprite
    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;
    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health = 0;
    }

    // Deal with collisions
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or if a collision ended
        if !event.pair.either_contains("player") || event.state.is_end() {
            continue;
        }
        if game_state.health > 0 {
            game_state.health -= 1;
            health_message.value = format!("Health: {}", game_state.health);
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.5);
        }
        if game_state.health == 0 {
            break;
        }
    }

    if game_state.health == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle1, 0.3);
    }
}

fn main() {
    // Create a game
    let mut game = Game::new();

    // Create the player sprite
    let player = game.add_sprite("player", SpritePreset::RacingCarRed);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;

    // Start some background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    // Create the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Create obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = rng().random_range(800.0..1600.0);
        obstacle.translation.y = rng().random_range(-300.0..300.0);
    }

    // Create the health message
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(0.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        health: 5,
        lost: false,
    });
}
