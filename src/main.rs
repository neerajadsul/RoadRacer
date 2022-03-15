use rusty_engine::prelude::*;

struct GameState {
    health: u8,
    lost: bool
}

/// player speed in pixels per second
const PLAYER_SPEED: f32 = 250;

fn main() {
    let mut game = Game::new();    
    let (window_width, window_height) = (600.0, 1000.0);
    //game setup
    let window_descriptor = WindowDescriptor {
        title: "Road Racer".into(),
        width: window_width,
        height: window_height,
        ..Default::default()
    };
    game.window_settings(window_descriptor);
    game.add_logic(game_logic);


    // player setup
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarRed);
    player1.translation.x = 0.0;
    player1.translation.y = -1.0 * window_height/2.0 + 100.0;
    player1.rotation = UP;
    player1.layer = 10.0;
    player1.collision = true;

    // game music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.10);

    game.run(GameState {
        health: 5,
        lost: false
    });

}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic
    let mut direction: f32 = 0.0;

    if engine.keyboard_state.just_pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Right) {
        direction += 1.0;
    }
}