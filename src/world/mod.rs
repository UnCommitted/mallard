/// world module - implements the asynchronous 'real' world of the running trains
use crate::telemetry::Telemetry;
use tokio::sync::mpsc::{Receiver, Sender};

// USE THE FOLLOWING FOR TESTING IF REQUIRED.
// use std::time::Duration;
// use tokio::time::delay_for;

/// World messaging enums

/// Commands to be sent to the world
#[derive(Debug)]
pub enum WorldCommand {
    AreYouReady,
    Quit,
}

/// Responses from world commands
#[derive(Debug)]
pub enum WorldResponse {
    WorldReady,
    ProcessedQuit,
}

/// World type, handles running the world we run trains in.
#[derive(Debug)]
pub struct World {
    pub command_ch: Receiver<WorldCommand>,
    pub response_ch: Sender<WorldResponse>,
    pub telemetry_ch: Sender<Telemetry>,
}

impl World {
    /// Main Async world function - this runs our train world
    pub async fn run(mut world: World) {
        // Main World Loop
        'main: loop {
            // Check for messages from the main thread
            match world.command_ch.try_recv() {
                // Respond to query asking for our readiness
                Ok(WorldCommand::AreYouReady) => {
                    println! {"Sending back world is ready"};
                    world
                        .response_ch
                        .send(WorldResponse::WorldReady)
                        .await
                        .unwrap();
                }

                // Respond to message to shut down world.
                Ok(WorldCommand::Quit) => {
                    println!("Got a quit message");
                    world
                        .response_ch
                        .send(WorldResponse::ProcessedQuit)
                        .await
                        .unwrap();

                    // Drop out of the main loop and exit the world
                    break 'main;
                }

                // Something has gone wrong..
                Err(_) => {}
            }

            // Send through some telemetry
            for i in 1..11 {
                world
                    .telemetry_ch
                    .send(Telemetry::BasicTelemetry(i))
                    .await
                    .unwrap();
            }
        }
    }
}
