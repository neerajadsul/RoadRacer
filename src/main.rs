use std::ops::RangeBounds;

use rusty_engine::prelude::*;

struct GameState {
    health: u8,
    lost: bool,
}

/// player speed in pixels per second
const PLAYER_SPEED: f32 = 250.0;
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
    
}
