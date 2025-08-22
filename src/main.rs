use std::thread;
use std::sync::mpsc;
use input::JoyDevice;

mod server;
mod input;
fn main(){
    let port= std::env::args().nth(1).unwrap_or(String::from("8007"));
    println!("port :{}",port);
    
    //device init
    let mut jdevice = input::JoyDevice::new();
    //network
    thread::spawn(||{server::broadcast()});
    let (tx, rx) = mpsc::channel();
    let port_clone = port.clone();
    thread::spawn(move || { 
    server::tcp_listener(&port_clone, tx);    
    
    });
   
    loop {
    if let Ok(x) = rx.try_recv(){
       JoyDevice::handle_input(&mut jdevice, x); 
    }
    }
    }
//a single input is being read multiple times leading to f key being pressed repeatedly






    


