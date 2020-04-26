use mallard::world::{World, WorldCommand, WorldResponse};
use tokio::sync::mpsc;
// use std::time::Duration;
// use tokio::time::delay_for;

/// railsim binary for testing instantiation of the various components
/// To be used as a template for later users.
#[tokio::main]
async fn main() {
    // Set up the communications with the world object
    // Channel for sending message from the main thread to the world
    let (mut main_sender, world_receiver) = mpsc::channel::<WorldCommand>(100);

    // Channel for sending messages from the world to the main thread
    let (world_sender, mut main_receiver) = mpsc::channel::<WorldResponse>(100);

    // Create our world object with the send and receive channels
    let temp_world = World {
        command_ch: world_receiver,
        response_ch: world_sender,
    };

    // Start the world
    let world_thread = tokio::spawn(World::run(temp_world));

    // Wait for the world to be ready
    println!("Seeing if world is ready");
    main_sender.send(WorldCommand::AreYouReady).await.unwrap();

    // Check that the world is ready
    match main_receiver.recv().await {
        Some(WorldResponse::WorldReady) => println!("World is ready"),
        Some(_) => {}
        None => {}
    }

    // Send the quit command to the world
    main_sender.send(WorldCommand::Quit).await.unwrap();

    // Loop through until we get a Quit Message.
    loop {
        match main_receiver.try_recv() {
            Ok(WorldResponse::ProcessedQuit) => {
                println!("Got ProcessedQuit from world, quitting");
                break;
            }
            Ok(WorldResponse::WorldReady) => {
                println!("World is ready");
            }
            Err(_) => {}
        }
    }

    world_thread.await.unwrap();
}
