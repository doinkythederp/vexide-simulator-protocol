use base64::{prelude::*, DecodeError};
use serde::{Deserialize, Serialize};
use std::{num::NonZeroU16, path::PathBuf};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Event {
    ScreenDraw {
        command: DrawCommand,
        color: Color,
    },
    ScreenClear {
        color: Color,
    },
    ScreenDoubleBufferMode {
        enable: bool,
    },
    ScreenRender,
    VCodeSig(VCodeSig),
    Ready,
    Exited,
    Serial {
        channel: i32,
        data: Vec<u8>,
    },
    DeviceUpdate {
        status: DeviceStatus,
        port: Port,
    },
    Battery(Battery),
    RobotPose {
        x: f64,
        y: f64,
    },
    RobotState(RobotState),
    Log {
        level: LogLevel,
        message: String,
    },
    VEXLinkConnect {
        port: SmartPort,
        id: String,
        mode: LinkMode,
        r#override: bool,
    },
    VEXLinkDisconnect {
        port: SmartPort,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Command {
    Touch {
        pos: Point,
        event: TouchEvent,
    },
    ControllerUpdate(ControllerUpdate),
    USD {
        root: Option<PathBuf>,
    },
    VEXLinkOpened {
        port: SmartPort,
        mode: LinkMode,
    },
    VEXLinkClosed {
        port: SmartPort,
    },
    CompetitionMode {
        connected: bool,
        mode: CompMode,
        is_competition: bool,
    },
    ConfigureDevice {
        port: Port,
        device: Device,
    },
    AdiInput {
        port: AdiPort,
        voltage: f64,
    },
    StartExecution,
    SetBatteryCapacity {
        capacity: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VCodeSig(pub String);

impl VCodeSig {
    pub fn new(bytes: &[u8]) -> Self {
        Self(BASE64_STANDARD.encode(bytes))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        BASE64_STANDARD.decode(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Device {
    Motor {
        physical_gearset: MotorGearset,
        moment_of_inertia: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RobotState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DrawCommand {
    Fill {
        shape: Shape,
    },
    Stroke {
        shape: Shape,
    },
    CopyBuffer {
        top_left: Point,
        bottom_right: Point,
        stride: NonZeroU16,
        /// Base64 string
        buffer: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Shape {
    Rectangle {
        top_left: Point,
        bottom_right: Point,
    },
    Circle {
        center: Point,
        radius: u16,
    },
    Pixel {
        pos: Point,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DeviceStatus {
    Motor {
        velocity: f64,
        reversed: bool,
        power_draw: f64,
        torque_output: f64,
        flags: MotorFlags,
        position: f64,
        target_position: f64,
        voltage: f64,
        gearset: MotorGearset,
        brake_mode: MotorBrakeMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MotorGearset {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MotorBrakeMode {
    Coast,
    Brake,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MotorFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LinkMode {
    Manager,
    Worker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TouchEvent {
    Released,
    Pressed,
    Held,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Port {
    Smart(SmartPort),
    Adi(AdiPort),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SmartPort(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AdiPort(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CompMode {
    Auto,
    Driver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Battery {
    pub voltage: f64,
    pub current: f64,
    pub capacity: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ControllerUpdate {
    /// Implementors can send raw controller state to the simulator,
    /// allowing for keyboard-and-mouse-based control.
    Raw(ControllerState),
    /// Or, they can send the UUID of a physical controller (more efficient and allows for SDL2 mappings).
    UUID(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ControllerState {
    pub axis1: i32,
    pub axis2: i32,
    pub axis3: i32,
    pub axis4: i32,
    pub button_l1: bool,
    pub button_l2: bool,
    pub button_r1: bool,
    pub button_r2: bool,
    pub button_up: bool,
    pub button_down: bool,
    pub button_left: bool,
    pub button_right: bool,
    pub button_x: bool,
    pub button_b: bool,
    pub button_y: bool,
    pub button_a: bool,
    pub button_sel: bool,
    pub battery_level: i32,
    pub button_all: bool,
    pub flags: i32,
    pub battery_capacity: i32,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
