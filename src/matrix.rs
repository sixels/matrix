use std::{io::Write, sync::mpsc, thread, time::Duration};

use crossterm::{cursor, execute, terminal::{self, ClearType}, queue, style::{SetBackgroundColor, Color, SetForegroundColor, Print}};

use crate::{
    event::{TermEvent, self},
    glyph::{self, Glyph},
    FPS,
};

/// The matrix state
pub struct Matrix {
    /// the output file (only stdout for now)
    out: Box<dyn Write>,

    /// terminal width
    width: u16,
    /// terminal height
    height: u16,

    /// the display buffer
    display: Vec<Vec<Glyph>>,
}

impl Matrix {
    /// Setup the screen and the initial state
    pub fn setup() -> Self {
        let mut stdout = std::io::stdout();

        // configure the terminal
        terminal::enable_raw_mode().unwrap();
        // I was having problems putting these two lines on the same `execute`
        execute!(stdout, terminal::EnterAlternateScreen).unwrap();
        execute!(stdout, cursor::Hide).unwrap();

        let (width, height) = terminal::size().expect("Could not get the terminal size");

        let mut rng = rand::thread_rng();
        let display = glyph::gen_glyphs(&mut rng, width, (width * 2).into());

        Self {
            out: Box::new(stdout),
            width,
            height,
            display,
        }
    }

    /// Make it rain! (press "q" to stop)
    pub fn rain(mut self) {
        let mut rng = rand::thread_rng();

        let (ev_tx, events) = mpsc::channel();

        let tx = ev_tx.clone();
        thread::spawn(move || redraw_loop(tx));

        let tx = ev_tx.clone();
        thread::spawn(move || event::handle_term_event(tx));

        let dt: f32 = 1.0 / FPS as f32;

        loop {
            match events.recv() {
                Ok(event) => match event {
                    TermEvent::Resize(w, h) => {
                        self.width = w;
                        self.height = h;
                    }
                    TermEvent::Redraw => {
                        queue!(
                            self.out,
                            terminal::Clear(ClearType::All),
                            SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 })
                        )
                        .unwrap();

                        self.display.iter_mut().for_each(|glyphs| {
                            let mut regen = false;
                            let tail_len = glyphs.len();
                            for (i, glyph) in glyphs.iter_mut().enumerate() {
                                glyph.y += glyph.vel * dt;

                                // don't render glyphs outside the screen
                                regen = glyph.y > self.height as f32;
                                if glyph.y <= 0.0 || regen {
                                    continue;
                                }

                                // x is implicitly greater than 0
                                let x = glyph.x as u16;
                                // we already asserted y is greater than 0
                                let y = glyph.y.ceil() as u16;

                                let fg = if i == 0 {
                                    Color::Rgb {
                                        r: 150,
                                        g: 255,
                                        b: 150,
                                    }
                                } else {
                                    Color::Rgb {
                                        r: 10,
                                        g: 255 - ((200/tail_len)*i) as u8,
                                        b: 10,
                                    }
                                };

                                queue!(
                                    self.out,
                                    cursor::MoveTo(x, y),
                                    SetForegroundColor(fg),
                                    Print(glyph.c),
                                    SetForegroundColor(Color::Reset)
                                )
                                .unwrap();
                            }
                            if regen {
                                *glyphs = glyph::gen_glyph(&mut rng, self.width);
                            }
                        });

                        self.out.flush().unwrap();
                    }
                    TermEvent::Exit => break,
                },
                Err(_) => break,
            }
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(self.out, terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    }
}

fn redraw_loop(tx: mpsc::Sender<TermEvent>) {
    loop {
        if let Err(_) = tx.send(TermEvent::Redraw) {
            break;
        }
        thread::sleep(Duration::from_millis(1) * 1000 / FPS as u32);
    }
}
