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
