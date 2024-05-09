use std::num::NonZeroU16;

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

pub enum Command {
    Touch {
        pos: Point,
        event: TouchEvent,
    },
    ControllerUpdate(ControllerUpdate),
    USD(USD),
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
}

pub struct VCodeSig;

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
        buffer: Vec<Color>,
    },
}

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

pub struct Point {
    x: i16,
    y: i16,
}

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

pub enum MotorGearset {
    Red,
    Green,
    Blue,
}

pub enum MotorBrakeMode {
    Coast,
    Brake,
    Hold,
}

pub struct MotorFlags;

pub enum LinkMode {
    Manager,
    Worker,
}

pub enum TouchEvent {
    Released,
    Pressed,
    Held,
}

pub enum Port {
    Smart(SmartPort),
    Adi(AdiPort),
}

pub struct SmartPort(u8);

pub struct AdiPort(u8);

pub enum CompMode {
    Auto,
    Driver,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
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
