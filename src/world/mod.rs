/// world module - implements the asynchronous 'real' world of the running trains
use tokio::sync::mpsc::{Receiver, Sender};

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
}

impl World {
    /// Main Async world function - this runs our train world
    pub async fn run(mut world: World) {
        loop {
            // Get message from the main thread
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
                    break;
                }

                // Something has gone wrong..
                Err(_) => {}
            }
        }
    }
}
