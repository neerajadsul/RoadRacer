use std::ops::RangeBounds;
use rand::prelude::*;
use SpritePreset::*;

use rusty_engine::prelude::*;

struct GameState {
    health: u8,
    lost: bool,
}

/// player speed in pixels per second
const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 1000.0;

fn main() {
    let mut game = Game::new();
    //game setup
    let window_descriptor = WindowDescriptor {
        title: "Road Racer".into(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    };
    game.window_settings(window_descriptor);
    game.add_logic(game_logic);

    // player setup
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarRed);
    player1.translation.x = 0.0;
    player1.translation.y = -1.0 * WINDOW_HEIGHT / 2.0 + 100.0;
    player1.rotation = UP;
    player1.layer = 10.0;
    player1.collision = true;

    for i in 1..15 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.rotation = UP;
        roadline.translation.y = WINDOW_HEIGHT - 150.0 * i as f32;
    }

    let obstacle_presets = vec![RacingBarrelBlue, RacingBarrelRed, RacingConeStraight];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.y = thread_rng().gen_range(-100.0..WINDOW_HEIGHT/2.0);
        obstacle.translation.x = thread_rng().gen_range(-280.0..280.0);
    }

    // create player health message
    let health_msg = game.add_text("health_msg", "Health: 5");
    health_msg.translation = Vec2::new(WINDOW_WIDTH/2.0 - 100.0, -WINDOW_HEIGHT/2.0 + 50.0);

    // game music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.10);

    game.run(GameState {
        health: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic
    let mut direction: f32 = 0.0;

    if engine.keyboard_state.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.x += (direction * PLAYER_SPEED * engine.delta_f32);
    if player1.translation.x > WINDOW_WIDTH / 2.0 {
        player1.translation.x = WINDOW_WIDTH / 2.0;
    }
    if player1.translation.x < -WINDOW_WIDTH / 2.0 {
        player1.translation.x = -WINDOW_WIDTH / 2.0;
    }    
    // player1.rotation = direction * 1.15;
    
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.y -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.y < (-WINDOW_HEIGHT/2.0) {
                sprite.translation.y = WINDOW_HEIGHT - 25.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.y -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.y < (-WINDOW_HEIGHT/2.0) {
                sprite.translation.y = thread_rng().gen_range(-100.0..WINDOW_HEIGHT/2.0);
                sprite.translation.x = thread_rng().gen_range(-280.0..280.0);
            }
        }
    }

    let health_msg = engine.texts.get_mut("health_msg").unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health > 0 {
            game_state.health -= 1;
            health_msg.value = format!("Health: {}", game_state.health);
            engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.45);
        }

    }

}
