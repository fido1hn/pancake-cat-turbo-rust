// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go!({
    let mut state = GameState::load();
    text!("Game state loaded");

    // Game play for player
    if gamepad(0).left.pressed() {
        state.cat_x -= 2.
    }

    if gamepad(0).right.pressed() {
        state.cat_x += 2.
    }

    // randomly spawned pancake
    if rand() % 64 == 0 {
        let pancake = Pankcake {
            x: (rand() % 256) as f32,
            y: 0.0,
            vel: (rand() % 3 + 1) as f32,
            radius: (rand() % 10 + 5) as f32,
        };

        state.pancakes.push(pancake);
    }

    state.save();
});

turbo::init! {
    struct GameState {
        frame: u32,
        last_much_at: u32,
        cat_x: f32,
        cat_y: f32,
        cat_r: f32,
        pancakes: Vec<struct Pankcake {
            x: f32,
            y: f32,
            vel: f32,
            radius:f32,
        }>,
        score: u32,
    } = Self {
        frame: 0,
        last_much_at: 0,
        cat_x: 128.0,
        cat_y: 112.0,
        cat_r:8.0,
        pancakes: vec![],
        score: 0
    }
}
