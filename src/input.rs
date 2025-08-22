use uinput::event::{absolute::PositionVariants, controller::{self, GamePad}, Controller};
use uinput::event::absolute::{Absolute, Position};



pub struct JoyDevice{
    pub device : uinput::Device
}


impl JoyDevice{ 

pub fn new() -> Self{
        
     let mut device = uinput::default()
        //this is called if device init is attempted but uinput module is not loaded by the system
        .expect("\x1b[31m FAILED TO CREATE DEVICE? MAKE SURE UINPUT IN LOADED!!!!
            \x1b[0m ").name("Joyer Controller")
        .expect("failed").event(uinput::event::Controller::All).expect(
            "faield")
        .create().expect("");
    
    return JoyDevice { device: device }
    }
pub fn handle_input(&mut self, code: String){
  let code = code.as_str().trim();

  if let Some(key) = code.strip_prefix("PR_"){
    dbg!(key);
    match key{
        "A" => { self.device.press(&Controller::GamePad(GamePad::South)).unwrap();
            self.device.synchronize().unwrap();},
        "B" => { self.device.press(&Controller::GamePad(GamePad::East)).unwrap();
            self.device.synchronize().unwrap()},
        "Y" => { self.device.press(&Controller::GamePad(GamePad::North)).unwrap();
            self.device.synchronize().unwrap();},
        "X" => { self.device.press(&Controller::GamePad(GamePad::West)).unwrap();
            self.device.synchronize().unwrap();},
        "LB" => { self.device.press(&Controller::GamePad(GamePad::TL)).unwrap();
                self.device.synchronize().unwrap();}
        "RB" => { self.device.press(&Controller::GamePad(GamePad::TR)).unwrap();
                self.device.synchronize().unwrap();}
        //DPAD STUFF
        "DPAD_DOWN" => {self.device.press(&Controller::DPad(controller::DPad::Down)).unwrap();
                self.device.synchronize().unwrap();
        dbg!("ddown pressed!");}
        "DPAD_UP" => {self.device.press(&Controller::DPad(controller::DPad::Up)).unwrap();
                self.device.synchronize().unwrap();}
        "DPAD_RIGHT" => {self.device.press(&Controller::DPad(controller::DPad::Right)).unwrap();
                self.device.synchronize().unwrap();}
        "DPAD_LEFT" => {self.device.press(&Controller::DPad(controller::DPad::Left)).unwrap();
                self.device.synchronize().unwrap()}
        _ => {println!("UNKNOWN INPUT!");}
        }

  }

  if let Some(key) = code.strip_prefix("RE_"){  
    dbg!(key);
    match key{
         "A" => { self.device.release(&Controller::GamePad(GamePad::South)).unwrap();
            self.device.synchronize().unwrap();},
        "B" => { self.device.release(&Controller::GamePad(GamePad::East)).unwrap();
            self.device.synchronize().unwrap()},
        "Y" => { self.device.release(&Controller::GamePad(GamePad::North)).unwrap();
            self.device.synchronize().unwrap();},
        "X" => { self.device.release(&Controller::GamePad(GamePad::West)).unwrap();
            self.device.synchronize().unwrap();},
        "LB" => { self.device.release(&Controller::GamePad(GamePad::TL)).unwrap();
                self.device.synchronize().unwrap();}
        "RB" => { self.device.release(&Controller::GamePad(GamePad::TR)).unwrap();
                self.device.synchronize().unwrap();}
        //dpad release 
        "DPAD_DOWN" => {self.device.release(&Controller::DPad(controller::DPad::Down)).unwrap();
                self.device.synchronize().unwrap();
                dbg!("DPAD DOWN PRESSED!");}
        "DPAD_UP" => {self.device.release(&Controller::DPad(controller::DPad::Up)).unwrap();
                self.device.synchronize().unwrap();}
        "DPAD_RIGHT" => {self.device.release(&Controller::DPad(controller::DPad::Right)).unwrap();
                self.device.synchronize().unwrap();}
        "DPAD_LEFT" => {self.device.release(&Controller::DPad(controller::DPad::Left)).unwrap();
                self.device.synchronize().unwrap()}



        _ => ()
    }
  }
}

}
