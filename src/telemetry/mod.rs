/// Telemetry module - Enums components can send back

/// World messaging enums

/// Commands to be sent to the world
#[derive(Debug)]
pub enum Telemetry {
    BasicTelemetry(i32),
}
