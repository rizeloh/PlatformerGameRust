use ggez::{Context, ContextBuilder, GameResult, event};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::input::keyboard::{self, KeyCode};

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    velocity_y: f32,
    on_ground: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 50.0,
            height: 60.0,
            velocity_y: 0.0,
            on_ground: false,
        }
    }

    fn update(&mut self) {
        if !self.on_ground {
            self.velocity_y += 0.5; // gravity
        }
        self.y += self.velocity_y;

        if self.y > 540.0 {
            self.y = 540.0;
            self.velocity_y = 0.0;
            self.on_ground = true;
        }
    }

    fn jump(&mut self) {
        if self.on_ground {
            self.velocity_y = -10.0;
            self.on_ground = false;
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.x, self.y, self.width, self.height),
            Color::from_rgb(0, 0, 255),
        )?;
        graphics::draw(ctx, &rectangle, DrawParam::default())
    }
}

struct Platform {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Platform {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.x, self.y, self.width, self.height),
            Color::from_rgb(0, 0, 0),
        )?;
        graphics::draw(ctx, &rectangle, DrawParam::default())
    }
}

struct MainState {
    player: Player,
    platforms: Vec<Platform>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let player = Player::new(100.0, 540.0);
        let platforms = vec![
            Platform::new(100.0, 590.0, 200.0, 10.0),
            Platform::new(400.0, 490.0, 200.0, 10.0),
            Platform::new(200.0, 390.0, 200.0, 10.0),
        ];
        let s = MainState { player, platforms };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(_ctx, KeyCode::Left) {
            self.player.x -= 5.0;
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Right) {
            self.player.x += 5.0;
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Space) {
            self.player.jump();
        }

        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(255, 255, 255));

        self.player.draw(ctx)?;

        for platform in &self.platforms {
            platform.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let (ctx, event_loop) = &mut ContextBuilder::new("platformer_game", "Author Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Platformer Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
