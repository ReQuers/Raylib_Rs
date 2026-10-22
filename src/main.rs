mod calculator;

use raylib::prelude::*;

const FONT_SIZE: i32 = 20;
const MAX_LEN: usize = 63;

#[derive(Default)]
struct TextField {
    text: String,
}

impl TextField {
    fn handle_input(&mut self, rl: &mut RaylibHandle) {
        while let Some(ch) = rl.get_char_pressed() {
            if (' '..='~').contains(&ch) && self.text.len() < MAX_LEN {
                self.text.push(ch);
            }
        }

        let erase = rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
            || rl.is_key_pressed_repeat(KeyboardKey::KEY_BACKSPACE);
        if erase {
            self.text.pop();
        }
    }
}

fn contains(rect: &Rectangle, p: Vector2) -> bool {
    p.x >= rect.x && p.x < rect.x + rect.width && p.y >= rect.y && p.y < rect.y + rect.height
}

struct ButtonState {
    hover: bool,
    pressed: bool,
    clicked: bool,
}

fn button_state(rl: &RaylibHandle, rect: &Rectangle) -> ButtonState {
    let hover = contains(rect, rl.get_mouse_position());
    ButtonState {
        hover,
        pressed: hover && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
        clicked: hover && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT),
    }
}

fn draw_button(
    d: &mut impl RaylibDraw,
    rect: Rectangle,
    label: &str,
    label_w: i32,
    state: &ButtonState,
) {
    let fill = if state.pressed {
        Color::GRAY
    } else if state.hover {
        Color::LIGHTGRAY
    } else {
        Color::RAYWHITE
    };
    d.draw_rectangle_rec(rect, fill);
    d.draw_rectangle_lines_ex(rect, 2.0, Color::DARKGRAY);

    let tx = (rect.x + (rect.width - label_w as f32) / 2.0) as i32;
    let ty = (rect.y + (rect.height - FONT_SIZE as f32) / 2.0) as i32;
    d.draw_text(label, tx, ty, FONT_SIZE, Color::BLACK);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(900, 900)
        .title("Rust + raylib GUI")
        .build();
    rl.set_target_fps(60);

    let field_rect = Rectangle {
        x: 20.0,
        y: 90.0,
        width: 400.0,
        height: 36.0,
    };
    let btn_rect = Rectangle {
        x: 430.0,
        y: 90.0,
        width: 150.0,
        height: 36.0,
    };
    let btn_label = "Calculate";
    let btn_label_w = rl.measure_text(btn_label, FONT_SIZE);

    let mut expr_field = TextField::default();
    let mut focused = false;
    let mut output_msg = String::from("Enter Input");

    while !rl.window_should_close() {
        // ---------- Update ----------
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            focused = contains(&field_rect, rl.get_mouse_position());
        }
        if focused {
            expr_field.handle_input(&mut rl);
        }

        let btn = button_state(&rl, &btn_rect);
        let pressed_enter = focused
            && (rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                || rl.is_key_pressed(KeyboardKey::KEY_KP_ENTER));

        if btn.clicked || pressed_enter {
            output_msg = match calculator::calculate(&expr_field.text) {
                Ok(value) => format!("Result : {value:.4}"),
                Err(err) => err.to_string(),
            };
        }

        let text_w = rl.measure_text(&expr_field.text, FONT_SIZE);
        let show_caret = focused && rl.get_time() % 1.0 < 0.5;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("Enter line", 20, 60, 16, Color::DARKGRAY);

        d.draw_rectangle_rec(field_rect, Color::WHITE);
        d.draw_rectangle_lines_ex(
            field_rect,
            2.0,
            if focused { Color::BLUE } else { Color::GRAY },
        );
        d.draw_text(&expr_field.text, 28, 98, FONT_SIZE, Color::BLACK);
        if show_caret {
            d.draw_text("_", 28 + text_w, 98, FONT_SIZE, Color::BLACK);
        }

        draw_button(&mut d, btn_rect, btn_label, btn_label_w, &btn);
        d.draw_text(&output_msg, 20, 160, FONT_SIZE, Color::DARKBLUE);
    }
}
