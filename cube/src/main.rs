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
    style::Color,
    symbols::Marker,
    terminal::{Frame, Terminal},
    widgets::{
        canvas::{Canvas, Line}, Block, BorderType, Borders, Widget
    },
};

mod cube;
use cube::Cube;

fn main() -> io::Result<()> {
    App::run()
}

struct App {
    cube: Cube,
    marker: Marker,
    pause: bool,
}

impl App {
    fn new() -> Self {
        Self {
            cube: Cube::new(200.0, [0.0, 0.0, 1000.0]),
            marker: Marker::Braille,
            pause: false,
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(1);
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
            self.cube.vertices = Cube::rot(&self.cube.vertices, 0.0015);
        }
    }

    fn ui(&self, frame: &mut Frame) {
        let area = frame.size();
        frame.render_widget(self.canvas_widget(), area);
    }

    fn canvas_widget(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(
                Block::default()
                .borders(Borders::all())
                .border_type(BorderType::Rounded)
            )
            .marker(self.marker)
            .paint(|ctx| {
                for line in self.get_lines() {
                    ctx.draw(&line);
                }
            })
            .x_bounds([10.0, 210.0])
            .y_bounds([10.0, 110.0])
    }
    
    fn get_lines(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let points = self.cube.get_projection(500.0);
        for edge in &self.cube.edges {
            let (x1,y1) = points[edge[0]];
            let (x2,y2) = points[edge[1]];
            let color = Color::LightBlue;
            lines.push(
                Line { x1, y1, x2, y2, color }
            )
        }
        lines
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