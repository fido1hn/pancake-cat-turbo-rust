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

    // Move falling pancakes and detect when they're caught
    let cat_center = (state.cat_x + state.cat_r, state.cat_y + state.cat_r);
    state.pancakes.retain_mut(|p| {
        p.y += p.vel;
        let dx = cat_center.0 - (p.x + p.radius);
        let dy = cat_center.1 - (p.y + p.radius);
        let distance = (dx * dx + dy * dy).sqrt();
        let radii_sum = state.cat_r + p.radius;
        let radii_diff = (state.cat_r - p.radius).abs();

        if radii_diff <= distance && distance <= radii_sum {
            state.score += 1;
            state.last_munch_at = state.frame;
            false
        } else if p.y < 144. + (p.radius * 2.) {
            true
        } else {
            false
        }
    });

    state.save();
});

turbo::init! {
    struct GameState {
        frame: u32,
        last_munch_at: u32,
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
        last_munch_at: 0,
        cat_x: 128.0,
        cat_y: 112.0,
        cat_r:8.0,
        pancakes: vec![],
        score: 0
    }
}
