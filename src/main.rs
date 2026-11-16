use macroquad::prelude::*;

use footy::{WINDOW_HEIGHT, WINDOW_WIDTH, draw};

fn window_config() -> Conf {
    Conf {
        window_title: "Footy".to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    loop {
        clear_background(BLACK);
        draw();
        next_frame().await;
    }
}
