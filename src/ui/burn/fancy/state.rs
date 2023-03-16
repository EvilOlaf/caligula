use std::time::Instant;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tracing::{info, trace};

use crate::{
    burn::{ipc::StatusMessage, state_tracking::ChildState},
    ui::burn::start::BeginParams,
};

use super::widgets::UIState;

#[derive(Debug, PartialEq, Clone)]
pub enum UIEvent {
    SleepTimeout,
    RecvChildStatus(Instant, Option<StatusMessage>),
    RecvTermEvent(Event),
}

#[derive(Debug, Clone)]
pub struct State {
    pub input_filename: String,
    pub target_filename: String,
    pub child: ChildState,
    pub ui_state: UIState,
}

impl State {
    pub fn initial(now: Instant, params: &BeginParams, input_file_bytes: u64) -> Self {
        State {
            input_filename: params.input_file.to_string_lossy().to_string(),
            target_filename: params.target.devnode.to_string_lossy().to_string(),
            ui_state: UIState::default(),
            child: ChildState::initial(now, &params, input_file_bytes),
        }
    }
    pub fn on_event(self, ev: UIEvent) -> anyhow::Result<Self> {
        trace!("Handling {ev:?}");

        Ok(match ev {
            UIEvent::SleepTimeout => self,
            UIEvent::RecvChildStatus(t, m) => Self {
                child: self.child.on_status(t, m),
                ..self
            },
            UIEvent::RecvTermEvent(e) => self.on_term_event(e)?,
        })
    }

    fn on_term_event(self, ev: Event) -> anyhow::Result<Self> {
        match ev {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                info!("Got CTRL-C, quitting");
                Err(Quit)?
            }
            _ => Ok(self),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("User sent quit signal")]
pub struct Quit;
