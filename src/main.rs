extern crate rlalloc;

use rlalloc::RAllocator;

#[global_allocator]
static GLOBAL: RAllocator = RAllocator::new();

use std::collections::HashSet;

use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
    pos_y: f32,
    keys: HashSet<KeyCode>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            pos_y: 0.0,
            keys: HashSet::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let sprinting =
            self.keys.contains(&KeyCode::LShift) || self.keys.contains(&KeyCode::RShift);
        let speed = if sprinting { 6. } else { 3. };
        self.keys.iter().for_each(|key| match key {
            KeyCode::W | KeyCode::Up => self.pos_y -= speed,
            KeyCode::S | KeyCode::Down => self.pos_y += speed,
            KeyCode::A | KeyCode::Left => self.pos_x -= speed,
            KeyCode::D | KeyCode::Right => self.pos_x += speed,
            _ => (),
        });
        self.pos_x = self.pos_x.clamp(0., ctx.gfx.size().0);
        self.pos_y = self.pos_y.clamp(0., ctx.gfx.size().1);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        canvas.draw(&circle, Vec2::new(self.pos_x, self.pos_y));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            self.keys.insert(keycode);
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            self.keys.remove(&keycode);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let v = vec![1, 2, 3];
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
