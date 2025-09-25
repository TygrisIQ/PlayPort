use uinput::event::{absolute::PositionVariants, controller::{self, GamePad}, Controller};
use uinput::event::absolute::{Absolute, Position};



pub struct JoyDevice{
    pub device : uinput::Device
}

impl Drop for JoyDevice{
    fn drop(&mut self){
        eprint!("DROPPING JOYDEVICE!");
    }
}
impl JoyDevice{ 

pub fn new() -> Self{
        
     let mut device = uinput::default()
        //this is called if device init is attempted but uinput module is not loaded by the system
        .expect("\x1b[31m FAILED TO CREATE DEVICE? MAKE SURE UINPUT IN LOADED!!!!
            \x1b[0m ").name("PlayPortDevice")
        .expect("failed").event(uinput::event::Controller::All).expect(
            "faield")
        .event(Absolute::Position(Position::X)).expect("ABS X")
        .event(Absolute::Position(Position::Y)).expect("ABS Y")
        .event(Absolute::Position(Position::RX)).expect("ABS RX")
        .event(Absolute::Position(Position::RY)).expect("ABS RY")
        .create().expect("");
    
    return JoyDevice { device: device }
    }


pub fn handle_input(&mut self, code: String){
  let code = code.as_str().trim();

  if let Some(key) = code.strip_prefix("PR_"){
    dbg!(key);
    match key{
        "A" => { self.device.press(&Controller::GamePad(GamePad::South)).unwrap();
            self.device.synchronize().unwrap();
            print!("\x1b[21m HEHEHEHEH \x1b[0m");
            },
        "B" => { self.device.press(&Controller::GamePad(GamePad::East)).unwrap();
            self.device.synchronize().unwrap();
            },
        "Y" => { self.device.press(&Controller::GamePad(GamePad::West)).unwrap();
            self.device.synchronize().unwrap();
            },
        "X" => { self.device.press(&Controller::GamePad(GamePad::North)).unwrap();
            self.device.synchronize().unwrap();
            },
        "LB" => { self.device.press(&Controller::GamePad(GamePad::TL)).unwrap();
                },
        "LB1" => { self.device.press(&Controller::GamePad(GamePad::TL2)).unwrap();}
        "RB" => { self.device.press(&Controller::GamePad(GamePad::TR)).unwrap();
                },
        "RB1" => { self.device.press(&Controller::GamePad(GamePad::TR2)).unwrap();}
        //DPAD STUFF
        "DPAD_DOWN" => {self.device.press(&Controller::DPad(controller::DPad::Down)).unwrap();
                
        dbg!("ddown pressed!");}
        "DPAD_UP" => {self.device.press(&Controller::DPad(controller::DPad::Up)).unwrap();
                }
        "DPAD_RIGHT" => {self.device.press(&Controller::DPad(controller::DPad::Right)).unwrap();
                }
        "DPAD_LEFT" => {self.device.press(&Controller::DPad(controller::DPad::Left)).unwrap();
            },
        "SELECT" => { self.device.press(&Controller::GamePad(GamePad::Select)).unwrap();},
        "START"  => { self.device.press(&Controller::GamePad(GamePad::Start)).unwrap();}
        _ => {println!("UNKNOWN INPUT!");}
        }
        self.device.synchronize().expect("DEVICE SYNC FAILED!");

  }

  if let Some(key) = code.strip_prefix("RE_"){  
    dbg!(key);
    match key{
         "A" => { self.device.release(&Controller::GamePad(GamePad::South)).unwrap();
            },
        "B" => { self.device.release(&Controller::GamePad(GamePad::East)).unwrap();
            },
        "Y" => { self.device.release(&Controller::GamePad(GamePad::West)).unwrap();
            },
        "X" => { self.device.release(&Controller::GamePad(GamePad::North)).unwrap();
            },
        "LB" => { self.device.release(&Controller::GamePad(GamePad::TL)).unwrap();
                },
        "LB1" => { self.device.release(&Controller::GamePad(GamePad::TL2)).unwrap();}
        "RB" => { self.device.release(&Controller::GamePad(GamePad::TR)).unwrap();
                },
        "RB1" => { self.device.release(&Controller::GamePad(GamePad::TR2)).unwrap();}
        //dpad release 
        "DPAD_DOWN" => {self.device.release(&Controller::DPad(controller::DPad::Down)).unwrap();
                
                dbg!("DPAD DOWN PRESSED!");}
        "DPAD_UP" => {self.device.release(&Controller::DPad(controller::DPad::Up)).unwrap();
                }
        "DPAD_RIGHT" => {self.device.release(&Controller::DPad(controller::DPad::Right)).unwrap();
                }
        "DPAD_LEFT" => {self.device.release(&Controller::DPad(controller::DPad::Left)).unwrap();
                },
        "SELECT" => {self.device.release(&Controller::GamePad(GamePad::Select)).unwrap();},
        "START" => {self.device.release(&Controller::GamePad(GamePad::Start)).unwrap();}


        _ => { println!("UNKNOWN INPUT!");}
    }
    self.device.synchronize().unwrap();
  }
  if let Some(ax) = code.strip_prefix("LS:"){
      if let Some((axis, v)) = ax.split_once(":"){
          if let Ok(mut v) = v.parse::<i32>(){
              v = v.clamp(-32768, 32767);
              match axis {
                  "X" => { let _ = self.device.position(&Position::X, v);},
                  "Y" => { let _ = self.device.position(&Position::Y, v);},
                  _ => {}
              }
              let _ = self.device.synchronize();
          }
      }
      return;
  }
  if let Some(ay) = code.strip_prefix("RS:"){
      if let Some((axis, v)) = ay.split_once(":"){
          if let Ok(mut v) = v.parse::<i32>(){
              v = v.clamp(-32768, 32767);
              match axis {
                  "X" => { let _ = self.device.position(&Position::RX, v);},
                  "Y" => { let _ = self.device.position(&Position::RY, v);},
                  _ => {}
              }
              let _ = self.device.synchronize();
          }
      }
      return;
  }
}

}
