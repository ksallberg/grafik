use nannou::prelude::*;

const SZ:f32 = 20.0;
const SP:f32 = 50.0;
const RAD:f32 = 250.0;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    mouse_position : Vec2,
    mouse_velocity : Vec2
}

fn model(app: &App) -> Model {
    Model {
        mouse_position: pt2(app.mouse().x, app.mouse().y),
        mouse_velocity: vec2(0.0, 0.0)
    }
}

fn update(app: &App, model: &mut Model) {
    let target = pt2(app.mouse().x, app.mouse().y);
    let difference = target - model.mouse_position;

    model.mouse_velocity += difference * 0.09;
    model.mouse_velocity *= 0.4;
    model.mouse_position += model.mouse_velocity;
}

fn view(app: &App, model: &Model, _window: Entity) {
    let draw = app.draw();
    draw.background().color(GREY);

    for i in 0..10 {
        for j in 0..10 {
            let frames = (app.elapsed_frames() % 100) as f32;
            let mut x:f32 = (j as f32) * SZ;
            x += (j as f32) * SP;
            x -= 200.0;

            let mut y:f32 = (i as f32) * SZ;
            y += (i as f32) * SP;
            y -= 300.0;

            let p_center = pt2(x + SZ/2.0, y + SZ/2.0);
            let mouse = model.mouse_position;
            let distance = p_center.distance(mouse);
            let direction = if distance > 0.0 {
                (p_center - mouse) / distance
            } else {
                vec2(0.0, 0.0)
            };
            let mut offset = direction * (RAD - distance) / 5.0;

            if distance < RAD {
                x += offset.x;
                y += offset.y;
            }

            draw.ellipse().x_y(x, y).radius(SZ/1.2).color(Color::WHITE);

            // Eyes
            if distance < RAD {
                offset *= -1.0;
            }

            let cur_eye_center = pt2(x,y);

            let eye_offset = (offset / 4.0)
                .clamp_length_max(SZ/1.2 - SZ/2.0);
            x = cur_eye_center.x + eye_offset.x;
            y = cur_eye_center.y + eye_offset.y;

            draw.ellipse().x_y(x, y).radius(SZ/2.0).color(BLUE);
            draw.ellipse().x_y(x, y).radius(SZ/3.0).color(Color::BLACK);
        }
    }
}
