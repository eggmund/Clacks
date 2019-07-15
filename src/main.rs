mod block;

use ggez::event;
use ggez::graphics::{self, DrawParam, Mesh, DrawMode, Rect};
//use ggez::timer;
use ggez::{Context, GameResult};

use crate::block::Block;

pub const FLOOR_POS: f32 = 500.0;
pub const ITERATIONS: usize = 1000;
const DELTA_TIME: f64 = 1.0/144.0;
const DELTA_PER_ITER: f64 = DELTA_TIME/ITERATIONS as f64;
const SMALL_BLOCK_SIZE: f64 = 20.0;
const LARGE_BLOCK_SIZE: f64 = 100.0;

struct MainState {
    small_block: Block,
    large_block: Block,
    collision_count: u64,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            small_block: Block::new(10.0, 0.0, 1.0, SMALL_BLOCK_SIZE, 0.0),
            large_block: Block::new(40.0, -2.0, (100.0_f64).powi(60), LARGE_BLOCK_SIZE, SMALL_BLOCK_SIZE),
            collision_count: 0,
        };
        Ok(s)
    }

    #[inline]
    fn was_collision(&mut self) {
        self.collision_count += 1;
    }

    fn draw_collision_count(&self, ctx: &mut Context) -> GameResult {
        use graphics::Text;
        let text = Text::new(format!("Collisions: {}", self.collision_count));

        graphics::draw(
            ctx,
            &text,
            DrawParam::default()
                .dest([10.0, 10.0])
                .color([0.0, 0.0, 0.0, 1.0].into())
        )
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //let dt = timer::duration_to_f64(timer::delta(ctx))/ITERATIONS as f64;

        for _ in 0..ITERATIONS {
            // Colliding if smaller block's (block[0]) right side is touching or through bigger one's left side
            let dist = (self.large_block.pos) - (self.small_block.pos + self.small_block.size as f64);
            if dist < 0.0 {
                let v1 = self.small_block.bounce(&self.large_block);
                let v2 = self.large_block.bounce(&self.small_block);
                self.small_block.vel = v1;
                self.large_block.vel = v2;
                self.was_collision();
            }

            if self.small_block.hit_wall(0.0) { self.was_collision(); }
            //self.large_block.hit_wall(0.0);

            self.small_block.update_pos(DELTA_PER_ITER);
            self.large_block.update_pos(DELTA_PER_ITER);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.85, 0.85, 0.85, 1.0].into());

        self.small_block.draw(ctx)?;
        self.large_block.draw(ctx)?;

        // Draw floor
        let floor = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, 10000.0, 1000.0),
            [0.0, 0.0, 0.0, 1.0].into()
        )?;

        graphics::draw(ctx, &floor, DrawParam::default().dest([0.0, FLOOR_POS]))?;

        self.draw_collision_count(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    use ggez::conf::{WindowSetup, WindowMode, NumSamples};

    let mut cb = ggez::ContextBuilder::new("Clacks", "eggmund")
        .window_setup(WindowSetup {
            title: "Clacks".to_owned(),
            samples: NumSamples::Two,
            vsync: true,
            ..Default::default()
        })
        .window_mode(WindowMode {
            width: 900.0,
            height: 600.0,
            ..Default::default()
        });

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}