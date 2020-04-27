use mallard::telemetry::Telemetry;
use mallard::world::{World, WorldCommand, WorldResponse};
use tokio::sync::mpsc;
use mallard::interface::run_gui;
// use std::time::Duration;
// use tokio::time::delay_for;

// Max telemetry messages before checking other channels.
const MAX_TELEMETRY_MESSAGES: i32 = 20;

/// railsim binary for testing instantiation of the various components
/// To be used as a template for later users.
#[tokio::main]
async fn main() {
    // Set up the communications with the world object
    // Channel for sending message from the main thread to the world
    let (mut main_sender, world_receiver) = mpsc::channel::<WorldCommand>(100);

    // Channel for sending messages from the world to the main thread
    let (world_sender, mut main_receiver) = mpsc::channel::<WorldResponse>(100);

    // Channel for receiving telemetry from the various components
    let (telemetry_sender, mut telemetry_receiver) = mpsc::channel::<Telemetry>(100);

    // Create our world object with the send and receive channels
    let temp_world = World {
        command_ch: world_receiver,
        response_ch: world_sender,
        telemetry_ch: telemetry_sender,
    };

    // Start the world
    let world_thread = tokio::spawn(World::run(temp_world));

    // Start the GUI
    let gui_thread = tokio::spawn(run_gui());

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

    // Main loop
    'main: loop {
        // Deal with world messages
        match main_receiver.try_recv() {
            Ok(WorldResponse::ProcessedQuit) => {
                println!("Got ProcessedQuit from world, quitting");
                break 'main;
            }
            Ok(WorldResponse::WorldReady) => {
                println!("World is ready");
            }
            Err(_) => {}
        }

        // Handle telemetry
        let mut telemetry_loop_ctr = 0;
        while telemetry_loop_ctr < MAX_TELEMETRY_MESSAGES {
            match telemetry_receiver.try_recv() {
                Ok(Telemetry::BasicTelemetry(number)) => {
                    println!("Got Some Basic Telemetry {}", number);
                }
                Err(_) => {
                    break;
                }
            }
            // Increment our loop counter to stop telemetry messages
            // from disallowing other actions.
            telemetry_loop_ctr += 1;
        }
        println!("Left the telemetry loop");
    }

    // Wait for the various threads to finish
    world_thread.await.unwrap();
    println!("World Simulation Thread has Finished");

    gui_thread.await.unwrap();
    println!("GUI Thread has Finished");

    println!("QUITTING THE SIMULATION");
}
