//! The Vexide Simulator Protocol enables communication between VEX robot simulators and user-facing frontends using a JSON-based protocol.
//!
//! The code executor and frontend communicate over a stream in [newline-delimited JSON format](https://jsonlines.org/).
//!
//! The backend sends [`Event`]s which represent a change in simulator state.
//! These are used by the frontend to correctly display the state of the simulated program.
//!
//! The frontend sends [`Command`]s to the code executor to control the robot code environment, simulating changes in robot hardware (like controller input and LCD touch events) or competition phase.
//!
//! The full protocol is documented at <https://internals.vexide.dev/simulators/protocol>.
#![deny(rust_2018_compatibility, rust_2018_idioms, unsafe_code)]

use base64::{prelude::*, DecodeError};
use mint::Point2;
use serde::{Deserialize, Serialize};
use std::{num::NonZeroU16, path::PathBuf};

/// A message sent from the simulator to the frontend.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Event {
    Handshake {
        version: i32,
        extensions: Vec<String>,
    },
    ScreenDraw {
        command: DrawCommand,
        color: Color,
        background: Color,
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
    TextMetricsRequest {
        text: String,
        options: V5Text,
    },
}

/// A message sent from the frontend to the simulator.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Command {
    Handshake {
        version: i32,
        extensions: Vec<String>,
    },
    Touch {
        pos: Point2<i16>,
        event: TouchEvent,
    },
    ControllerUpdate(Option<ControllerUpdate>, Option<ControllerUpdate>),
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
        enabled: bool,
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
    SetTextMetrics {
        text: String,
        options: V5Text,
        metrics: TextMetrics,
    },
}

/// Base64-encoded program metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VCodeSig(pub String);

impl VCodeSig {
    pub fn new(bytes: &[u8]) -> Self {
        Self(BASE64_STANDARD.encode(bytes))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        BASE64_STANDARD.decode(&self.0)
    }
}

/// The configuration of a V5 peripheral.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Device {
    Motor {
        physical_gearset: MotorGearset,
        moment_of_inertia: f64,
    },
}

/// The current state of the robot as a whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RobotState;

/// An instruction for drawing to the robot LCD screen.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DrawCommand {
    Fill {
        shape: Shape,
    },
    Stroke {
        shape: Shape,
    },
    CopyBuffer {
        top_left: Point2<i16>,
        bottom_right: Point2<i16>,
        stride: NonZeroU16,
        /// Base64 string
        buffer: String,
    },
    Write {
        text: V5Text,
        coordinates: Point2<i16>,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct V5Text {
    pub data: String,
    pub font_family: V5FontFamily,
    pub font_size: V5FontSize,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
)]
pub enum V5FontFamily {
    #[default]
    UserMono,
    TimerMono,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
)]
pub enum V5FontSize {
    Small,
    #[default]
    Normal,
    Large,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
)]
pub struct TextMetrics {
    width: usize,
    height: usize,
}

/// A shape that can be drawn to the robot LCD screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Shape {
    Rectangle {
        top_left: Point2<i16>,
        bottom_right: Point2<i16>,
    },
    Circle {
        center: Point2<i16>,
        radius: u16,
    },
    Pixel {
        pos: Point2<i16>,
    },
}

/// The current state of a V5 peripheral.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeviceStatus {
    Motor {
        velocity: f64,
        reversed: bool,
        power_draw: f64,
        torque_output: f64,
        flags: i32,
        position: f64,
        target_position: f64,
        voltage: f64,
        gearset: MotorGearset,
        brake_mode: MotorBrakeMode,
    },
}

/// The gearset of a VEX V5 motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MotorGearset {
    Red,
    Green,
    Blue,
}

/// The brake mode of a VEX V5 motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MotorBrakeMode {
    Coast,
    Brake,
    Hold,
}

/// The mode of a [VEXlink](https://drive.google.com/file/d/13mTA6BT7CPskJzh4YgsfAfoH9OgK75Hn/view)-configured radio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LinkMode {
    Manager,
    Worker,
}

/// The gearset of a VEX V5 motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TouchEvent {
    Released,
    Pressed,
    Held,
}

/// An arbitrary port on the VEX V5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Port {
    Smart(SmartPort),
    Adi(AdiPort),
}

/// An RJ9 4p4c "Smart" port on the VEX V5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SmartPort(pub u8);

/// A 3-wire "ADI" port for analog devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AdiPort(pub u8);

/// The current stage of a competition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CompMode {
    Auto,
    Driver,
}

/// An RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// The importance level of a log message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

/// Battery status and statistics.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Battery {
    pub voltage: f64,
    pub current: f64,
    pub capacity: f64,
}

/// A method of retrieving a controller's current state.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ControllerUpdate {
    /// Implementors can send raw controller state to the simulator,
    /// allowing for keyboard-and-mouse-based control.
    Raw(ControllerState),
    /// Implementors can can send the UUID of a physical controller (more efficient and allows for SDL2 mappings).
    UUID(String),
}

/// The raw state of a VEX V5 controller.
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
