use macroquad::prelude::*;

use footy::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, run};

fn window_config() -> Conf {
    Conf {
        window_title: "Footy".to_string(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    run().await;
}
