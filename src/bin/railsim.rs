// Mallard specific library
use mallard::interface::FromInterfaceMessage;
use mallard::telemetry::Telemetry;
use mallard::world::{World, WorldCommand, WorldResponse};

// GUI
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

// Threading and time control
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

// Max telemetry messages before checking other channels.
const MAX_TELEMETRY_MESSAGES: i32 = 20;

/// railsim binary for testing instantiation of the various components
/// To be used as a template for later users.
#[tokio::main]
async fn main() {
    // Set up the communications with the world object
    // Channel for sending message from the main thread to the world
    let (mut main_world_sender, world_receiver) = mpsc::channel::<WorldCommand>(100);

    // Channel for sending messages from the world to the main thread
    let (world_sender, mut main_world_receiver) = mpsc::channel::<WorldResponse>(100);

    // Channel for recieving telemetry from the world
    // Channel for receiving telemetry from the various components
    let (telemetry_sender, mut telemetry_receiver) = mpsc::channel::<Telemetry>(100);

    // Start the world
    let world_thread = tokio::spawn(mallard::world::run_world(World {
        command_ch: world_receiver,
        response_ch: world_sender,
        telemetry_ch: telemetry_sender,
    }));

    // Wait for the world to be ready
    println!("Seeing if world is ready");
    main_world_sender
        .send(WorldCommand::AreYouReady)
        .await
        .unwrap();

    // Check that the world is ready
    match main_world_receiver.recv().await {
        Some(WorldResponse::WorldReady) => println!("World is ready"),
        Some(_) => {}
        None => {}
    }

    // Set up the gui
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    // Main loop duration
    let max_time = Duration::from_millis(15);

    // Main loop
    'main: loop {
        let start = Instant::now();

        // Start a delay to check for at the end of the loop
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    main_world_sender.send(WorldCommand::Quit).await.unwrap();
                    // break 'main;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();

        // Deal with world messages
        match main_world_receiver.try_recv() {
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
        while Instant::now().duration_since(start) < max_time {
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

    println!("QUITTING THE SIMULATION");
}
