use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Hello raylib").build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("Hello, raylib!", 280, 200, 30, Color::DARKGRAY);
        d.draw_fps(10, 10);
    }
}
