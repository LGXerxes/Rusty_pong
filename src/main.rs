use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};

const PADDING: f32 = 40.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 200.0;
const BALL_SPEED_MULTI: f32 = 1.05;
const PLAYER_SPEED: f32 = 300.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}
fn move_racket(
    pos: &mut na::Point2<f32>,
    keycode: KeyCode,
    y_dir: f32,
    ctx: &mut Context,
    dt: f32,
) {
    let screen_h = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, keycode) {
        pos.y += y_dir * PLAYER_SPEED * dt
    }
    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_h - RACKET_HEIGHT_HALF,
    );
}
fn randomize_vector(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vector(&mut ball_vel, BALL_SPEED, BALL_SPEED);
        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF + PADDING, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF - PADDING, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_vel,
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let screen_w_half = screen_w * 0.5;
        //Moving the rackets
        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx, dt);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx, dt);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx, dt);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx, dt);

        self.ball_pos.x += self.ball_vel.x * dt;
        self.ball_pos.y += self.ball_vel.y * dt;

        if self.ball_pos.x < 0.0 {
            let mut rng = thread_rng();
            self.ball_pos.x = screen_w_half;
            self.ball_pos.y = rng.gen_range(0, screen_h as i32) as f32;
            randomize_vector(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }
        if self.ball_pos.x > screen_w {
            let mut rng = thread_rng();
            self.ball_pos.x = screen_w_half;
            self.ball_pos.y = rng.gen_range(0, screen_h as i32) as f32;
            randomize_vector(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y *= -1.0;
            self.ball_vel *= BALL_SPEED_MULTI;
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y *= -1.0;
            self.ball_vel *= BALL_SPEED_MULTI;
        }

        let intersetcs_player_1 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersetcs_player_1 {
            self.ball_vel.x *= -1.0
        }
        let intersetcs_player_2 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersetcs_player_2 {
            self.ball_vel.x *= -1.0
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;

        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text = ggez::graphics::Text::new(format!(
            "{}         {}",
            self.player_1_score, self.player_2_score
        ));
        let screen_w = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;
        let mut score_pos = na::Point2::new(screen_w_half, 20.0);
        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);
        draw_param.dest = score_pos.into();
        graphics::draw(ctx, &score_text, draw_param)?;
        graphics::present(ctx)?;
        Ok(())
    }
}
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "TanTan");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PONG");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)?;

    Ok(())
}
