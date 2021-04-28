use ggez::{
    conf,
    event::{self, EventHandler},
    Context, GameResult,
};
use orst::algos::bubblesort::BubbleSort;

const APP_WIDTH: f32 = 1280.0;
const APP_HEIGHT: f32 = 720.0;

struct AppState {}

impl AppState {
    pub fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut things = vec![3, 2, 4, 1, 6];

        let bub = orst::sort::<_, BubbleSort>(&mut things);

        println!("{:?}", things);

        let s = AppState {};

        Ok(s)
    }
}

impl EventHandler for AppState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        todo!()
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(true))
        .window_mode(conf::WindowMode::default().dimensions(APP_WIDTH as f32, APP_HEIGHT as f32));

    let (mut ctx, mut events_loop) = cb.build()?;

    let mut app = AppState::new(&mut ctx)?;

    event::run(&mut ctx, &mut events_loop, &mut app)
}
