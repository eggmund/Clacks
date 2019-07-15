//extern crate nalgebra as na;
extern crate nannou;

mod block;

use nannou::prelude::*;
use nannou::geom::Point2;

use crate::block::Block;

pub const FLOOR_HEIGHT: f32 = -100.0;
pub const ITERATIONS: usize = 5000000;
const SCREEN_DIMS: (f32, f32) = (1000.0, 800.0);

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    small_block: Block,
    large_block: Block,
    collision_count: u64,
    floor_rect: nannou::geom::rect::Rect,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_dimensions(SCREEN_DIMS.0 as u32, SCREEN_DIMS.1 as u32)
        .view(view)
        .build()
        .unwrap();

    Model {
        small_block: Block::new(-10.0, 0.0, 1.0, 20.0),
        large_block: Block::new(100.0, -100.0, (100.0).powi(5), 100.0),

        collision_count: 0,

        floor_rect: nannou::geom::rect::Rect::from_corners(Point2::new(-SCREEN_DIMS.0/2.0, FLOOR_HEIGHT), Point2::new(SCREEN_DIMS.0/2.0, -SCREEN_DIMS.1/2.0)),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let half_screen_w = app.window_rect().w()/2.0;
    let dt = update.since_last.secs()/ITERATIONS as f64;

    for _ in 0..ITERATIONS {
        // Colliding if smaller block's (block[0]) right side is touching or through bigger one's left side
        let dist = (model.large_block.pos - model.large_block.size as f64/2.0) - (model.small_block.pos + model.small_block.size as f64/2.0);
        if dist < 0.0 {
            let v1 = model.small_block.bounce(&model.large_block);
            let v2 = model.large_block.bounce(&model.small_block);
            model.small_block.vel = v1;
            model.large_block.vel = v2;
            model.collision_count += 1;
            println!("Collisions: {}", model.collision_count);
        }

        if model.small_block.hit_wall(half_screen_w) { model.collision_count += 1; println!("Collisions: {}", model.collision_count); }
        model.large_block.hit_wall(half_screen_w);

        model.small_block.update_pos(dt);
        model.large_block.update_pos(dt);
    }

    // let mut ui = model.ui.set_widgets();
    // widget::Text::new(&format!("Collisions: {}", model.collision_count))
    //     .top_left_with_margin(20.0)
    //     .color(color::RED)
    //     .set(model.text, &mut ui);
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.rect()
        .x_y(model.floor_rect.x(), model.floor_rect.y())
        .w_h(model.floor_rect.w(), model.floor_rect.h())
        .color(BLACK);

    model.small_block.display(&draw);
    model.large_block.display(&draw);

    draw.to_frame(app, &frame).unwrap();
    frame
}