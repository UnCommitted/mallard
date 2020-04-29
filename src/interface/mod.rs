/// Implements the graphical user interface using SDL2
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::Duration;

// use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::Sender;

// use tokio::time::delay_for;

// Messages that can be passed to and from the interface
mod messages;

// Expose some messages to the main sim loop
pub use crate::interface::messages::FromInterfaceMessage;

/// GUI type, holds the channels for communication with the main thread
#[derive(Debug)]
pub struct GUI {
    // pub command_ch: Receiver<WorldCommand>,
    pub response_ch: Sender<FromInterfaceMessage>,
    // pub telemetry_ch: Receiver<Telemetry>,
}

pub async fn run_gui() {}
