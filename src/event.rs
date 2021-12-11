use std::sync::mpsc;

use crossterm::event::{self, KeyEvent, KeyCode};

pub enum TermEvent {
    Redraw,
    Resize(u16, u16),
    Exit,
}

pub fn handle_term_event(ev_tx: mpsc::Sender<TermEvent>) {
    loop {
        let result = match event::read().unwrap() {
            event::Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => Some(ev_tx.send(TermEvent::Exit)),
            event::Event::Resize(w, h) => Some(ev_tx.send(TermEvent::Resize(w, h))),
            _ => None,
        };

        // break if the receiver is dropped
        if let Some(Err(_)) = result {
            break;
        }
    }
}
