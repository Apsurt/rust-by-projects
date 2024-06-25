use std::{
    io::{self, stdout, Stdout}, time::{Duration, Instant}
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    style::Color,
    symbols::Marker,
    terminal::{Frame, Terminal},
    widgets::{
        canvas::{Canvas, Circle, Points},
        Block, Widget,
    },
};

mod cube;
use cube::Cube;

fn main() {// -> io::Result<()> {
    
    let app = App::new();
    //App::run()
}

struct App {
    ball: Circle,
    cube: Cube,
    playground: Rect,
    vx: f64,
    vy: f64,
    marker: Marker,
    pause: bool,
}

impl App {
    fn new() -> Self {
        Self {
            ball: Circle {
                x: 40.0,
                y: 80.0,
                radius: 20.0,
                color: Color::LightRed,
            },
            cube: Cube::new(1.0),
            playground: Rect::new(10, 10, 200, 100),
            vx: 1.0,
            vy: 1.0,
            marker: Marker::Braille,
            pause: false,
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => app.pause = !app.pause,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {
        if !self.pause {
            let ball = &self.ball;
            let playground = self.playground;
            if ball.x - ball.radius < f64::from(playground.left())
                || ball.x + ball.radius > f64::from(playground.right())
            {
                self.vx = -self.vx;
            }
            if ball.y - ball.radius < f64::from(playground.top())
                || ball.y + ball.radius > f64::from(playground.bottom())
            {
                self.vy = -self.vy;
            }
            
            self.ball.x += self.vx;
            self.ball.y += self.vy;
        }
    }

    fn ui(&self, frame: &mut Frame) {
        let area = frame.size();
        frame.render_widget(self.canvas_widget(), area);
    }

    fn canvas_widget(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered())
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball);
            })
            .x_bounds([10.0, 210.0])
            .y_bounds([10.0, 110.0])
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}