use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Margin},
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::{
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
        Wrap,
    },
    Terminal,
};

use crate::pipe::{build_surface, Coords, Surface};

pub fn run_tui(input: &str) -> Result<()> {
    chain_hook();
    let terminal = init_terminal()?;

    let mut app = AppBuilder::default()
        .with_surface_source(input)?
        .with_tick_rate(Duration::from_millis(1))
        .build();

    app.run_tui(terminal)?;

    restore_terminal()?;
    Ok(())
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

pub(super) fn chain_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        restore_terminal().unwrap();
        original_hook(panic);

        eprintln!("Unrecoverable error...");
        eprintln!("Terminal restored...");
        eprintln!("Original panic below...");
    }));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Stopped,
    #[default]
    Running,
}

pub struct App {
    surface: Surface,
    timer: Instant,
    vertical_scroll: usize,
    state: AppState,
    tick_rate: Duration,
    log: String,
}

impl App {
    pub fn run_tui(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        let mut last_tick = Instant::now();

        let termination_flag = Arc::new(AtomicBool::new(false));
        let flag_copy = Arc::clone(&termination_flag);
        ctrlc::set_handler(move || flag_copy.store(true, Ordering::Relaxed))?;

        while self.is_running() {
            // Check SIG... hooks if we need to exit
            if termination_flag.load(Ordering::Relaxed) {
                self.quit();
            }

            // Guess what
            self.draw(&mut terminal)?;

            // Handle events with a minor delay between handler executions
            if last_tick.elapsed() >= self.tick_rate {
                let timeout = self.tick_rate.saturating_sub(last_tick.elapsed());
                self.handle_events(Some(timeout))?;
            }
            // Autoupdate the underlying struct every TICK_RATE
            if last_tick.elapsed() >= self.tick_rate {
                self.update();
                last_tick = Instant::now();
            }

            // // Sleep for a short duration to avoid busy waiting
            // std::thread::sleep(self.tick_rate);
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        //  // Main render
        // terminal.draw(|frame| self.ui(frame))?;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn quit(&mut self) {
        self.state = AppState::Stopped;
    }

    fn handle_events(&mut self, timeout: Option<Duration>) -> Result<()> {
        let event_available = match timeout {
            // Nonblocking timeout
            Some(timeout) => event::poll(timeout)?,
            // Blocking event reader
            None => true,
        };

        if event_available {
            // Blocking thread until Event is available
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Char(' ') => self.update(),
                    KeyCode::Char('p') => panic!("intentional demo panic"),
                    KeyCode::Down => {
                        if let Some(scroll) = self.vertical_scroll.checked_add(1) {
                            self.vertical_scroll = scroll;
                        };
                    }
                    KeyCode::Up => {
                        if let Some(scroll) = self.vertical_scroll.checked_sub(1) {
                            self.vertical_scroll = scroll;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn update(&mut self) {
        self.surface.update();
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Length(8),
        ]);
        let area = layout.areas::<3>(area);
        // Main map
        self.text().render(area[0], buf);
        //Scrollbar
        let (scrollbar, mut scrollbar_state) = self.scrollbar();
        scrollbar.render(
            area[0].inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            buf,
            &mut scrollbar_state,
        );
        // Statusbar
        self.status_bar().render(area[1], buf);
        // Log
        self.log().render(area[2], buf);
    }
}

// Widgets implementation
impl App {
    fn text(&self) -> Paragraph {
        // Main text paragraph
        let text = self
            .surface
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, surface)| match surface {
                        crate::pipe::SurfaceType::Pipe(_) => {
                            let coords = Coords { row: i, col: j };
                            if self.surface.search().visited().contains(&coords) {
                                surface.to_span().green()
                            } else if Some(&coords) == self.surface.search().queue().peek() {
                                surface.to_span().red()
                            } else if self.surface.search().queue().contains(&coords) {
                                surface.to_span().yellow()
                            } else {
                                surface.to_span().white()
                            }
                        }
                        crate::pipe::SurfaceType::Ground => surface.to_span().yellow(),
                        crate::pipe::SurfaceType::StartingPositon => {
                            surface.to_span().green().bold().rapid_blink()
                        }
                    })
                    .collect::<Line>()
            })
            .collect::<Text>();

        Paragraph::new(text)
            .scroll((self.vertical_scroll as u16, 0))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Thick)
                    .title("Surface")
                    .blue(),
            )
            .wrap(Wrap { trim: false })
    }

    fn status_bar(&self) -> Paragraph {
        let text = vec![
            Line::from(format!(
                "Current counter: {:<.3} | Vscroll: {} | Tick rate: {:<.3} sec",
                self.timer.elapsed().as_secs_f32(),
                self.vertical_scroll,
                self.tick_rate.as_secs_f32()
            )),
            Line::from("press `p` to panic"),
        ];

        Paragraph::new(text)
            .block(Block::bordered().title("Test info"))
            .centered()
    }

    fn log(&self) -> Paragraph {
        Paragraph::new(self.log.clone()).block(Block::bordered().title("Log"))
    }

    fn scrollbar(&self) -> (Scrollbar, ScrollbarState) {
        // Scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let scrollbar_state =
            ScrollbarState::new(self.surface.len()).position(self.vertical_scroll);

        (scrollbar, scrollbar_state)
    }
}

#[derive(Default)]
pub struct AppBuilder {
    surface: Surface,
    tick_rate: Duration,
}

impl AppBuilder {
    pub fn with_surface_source(mut self, surface_source: &str) -> Result<Self> {
        self.surface = build_surface(surface_source)?;
        Ok(self)
    }

    pub fn with_tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = tick_rate;
        self
    }

    pub fn build(self) -> App {
        let _width = match &self.surface.first() {
            Some(row) => row.len(),
            None => 0,
        };
        let _height = self.surface.len();

        App {
            surface: self.surface,
            timer: Instant::now(),
            vertical_scroll: 0,
            state: AppState::Running,
            tick_rate: match self.tick_rate.is_zero() {
                true => Duration::from_millis(16),
                false => self.tick_rate,
            },
            log: String::new(),
        }
    }
}
