use uinput::event::{controller::{self, GamePad}, Controller};
use uinput::event::absolute::{Absolute, Position};

// Binary protocol:
// Buttons:  [msg_type: u8] [button_id: u8]             = 2 bytes
// Axes:     [msg_type: u8] [axis_id: u8] [value: i16]  = 4 bytes
//
// msg_type:
//   0x01 = PRESS
//   0x02 = RELEASE
//   0x03 = AXIS
//
// button_id:
//   0x00 = A (South)
//   0x01 = B (East)
//   0x02 = X (North)
//   0x03 = Y (West)
//   0x04 = LB  (TL)
//   0x05 = LB1 (TL2)
//   0x06 = RB  (TR)
//   0x07 = RB1 (TR2)
//   0x08 = DPAD_UP
//   0x09 = DPAD_DOWN
//   0x0A = DPAD_LEFT
//   0x0B = DPAD_RIGHT
//   0x0C = SELECT
//   0x0D = START
//
// axis_id:
//   0x00 = LS_X
//   0x01 = LS_Y
//   0x02 = RS_X
//   0x03 = RS_Y

const PRESS:   u8 = 0x01;
const RELEASE: u8 = 0x02;
const AXIS:    u8 = 0x03;

const BTN_A:          u8 = 0x00;
const BTN_B:          u8 = 0x01;
const BTN_X:          u8 = 0x02;
const BTN_Y:          u8 = 0x03;
const BTN_LB:         u8 = 0x04;
const BTN_LB1:        u8 = 0x05;
const BTN_RB:         u8 = 0x06;
const BTN_RB1:        u8 = 0x07;
const BTN_DPAD_UP:    u8 = 0x08;
const BTN_DPAD_DOWN:  u8 = 0x09;
const BTN_DPAD_LEFT:  u8 = 0x0A;
const BTN_DPAD_RIGHT: u8 = 0x0B;
const BTN_SELECT:     u8 = 0x0C;
const BTN_START:      u8 = 0x0D;

const AXIS_LS_X: u8 = 0x00;
const AXIS_LS_Y: u8 = 0x01;
const AXIS_RS_X: u8 = 0x02;
const AXIS_RS_Y: u8 = 0x03;

pub struct JoyDevice {
    pub device: uinput::Device
}

impl Drop for JoyDevice {
    fn drop(&mut self) {
        eprint!("DROPPING JOYDEVICE!");
    }
}

impl JoyDevice {

    pub fn new() -> Self {
        let device = uinput::default()
            .expect("\x1b[31m FAILED TO CREATE DEVICE? MAKE SURE UINPUT IS LOADED!\x1b[0m")
            .name("PlayPortDevice")
            .expect("failed to set device name")
            .event(uinput::event::Controller::All)
            .expect("failed to register controller events")
            .event(Absolute::Position(Position::X)).expect("ABS X")
            .event(Absolute::Position(Position::Y)).expect("ABS Y")
            .event(Absolute::Position(Position::RX)).expect("ABS RX")
            .event(Absolute::Position(Position::RY)).expect("ABS RY")
            .create()
            .expect("failed to create uinput device");

        JoyDevice { device }
    }

    pub fn handle_input(&mut self, packet: Vec<u8>) {
        if packet.is_empty() { return; }

        eprintln!("[INPUT] raw packet: {:?}", packet);

        match packet[0] {
            PRESS | RELEASE if packet.len() >= 2 => {
                let pressing = packet[0] == PRESS;
                eprintln!("[INPUT] {} button id={:#04x}", if pressing { "PRESS" } else { "RELEASE" }, packet[1]);
                self.handle_button(packet[1], pressing);
                self.device.synchronize().expect("DEVICE SYNC FAILED!");
            }
            AXIS if packet.len() >= 4 => {
                let value = i16::from_le_bytes([packet[2], packet[3]]) as i32;
                eprintln!("[INPUT] AXIS id={:#04x} value={}", packet[1], value);
                self.handle_axis(packet[1], value);
                let _ = self.device.synchronize();
            }
            _ => { eprintln!("[INPUT] UNKNOWN PACKET: {:?}", packet); }
        }
    }

    fn handle_button(&mut self, id: u8, pressing: bool) {
        let event = match id {
            BTN_A          => Controller::GamePad(GamePad::South),
            BTN_B          => Controller::GamePad(GamePad::East),
            BTN_X          => Controller::GamePad(GamePad::North),
            BTN_Y          => Controller::GamePad(GamePad::West),
            BTN_LB         => Controller::GamePad(GamePad::TL),
            BTN_LB1        => Controller::GamePad(GamePad::TL2),
            BTN_RB         => Controller::GamePad(GamePad::TR),
            BTN_RB1        => Controller::GamePad(GamePad::TR2),
            BTN_DPAD_UP    => Controller::DPad(controller::DPad::Up),
            BTN_DPAD_DOWN  => Controller::DPad(controller::DPad::Down),
            BTN_DPAD_LEFT  => Controller::DPad(controller::DPad::Left),
            BTN_DPAD_RIGHT => Controller::DPad(controller::DPad::Right),
            BTN_SELECT     => Controller::GamePad(GamePad::Select),
            BTN_START      => Controller::GamePad(GamePad::Start),
            _              => { eprintln!("UNKNOWN BUTTON ID: {:#04x}", id); return; }
        };

        if pressing {
            self.device.press(&event).unwrap();
        } else {
            self.device.release(&event).unwrap();
        }
    }

    fn handle_axis(&mut self, id: u8, value: i32) {
        let value = value.clamp(-32768, 32767);
        match id {
            AXIS_LS_X => { let _ = self.device.position(&Position::X,  value); }
            AXIS_LS_Y => { let _ = self.device.position(&Position::Y,  value); }
            AXIS_RS_X => { let _ = self.device.position(&Position::RX, value); }
            AXIS_RS_Y => { let _ = self.device.position(&Position::RY, value); }
            _         => { eprintln!("UNKNOWN AXIS ID: {:#04x}", id); }
        }
    }

}