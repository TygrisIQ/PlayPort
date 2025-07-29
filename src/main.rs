use std::{net::{TcpListener, TcpStream}, thread};
use std::io::Read;
use std::sync::mpsc;

mod server;
fn main(){
    let port= std::env::args().nth(1).unwrap_or(String::from("8007"));
    println!("port :{}",port);
    let device = uinput::default().expect("default failed")
        .name("test").expect("name failed")
        .event(uinput::event::Keyboard::All).expect("event failed")
        .create().expect("Creation failed");
    //network
    thread::spawn(||{server::broadcast()});
    let (tx, rx) = mpsc::channel();
    let port_clone = port.clone();
    thread::spawn(move || {
         
    let code = server::tcp_listener(&port_clone).unwrap(); 
    tx.send(code).unwrap();
        
    });
    if let Ok(code) = rx.recv(){
        println!("Code is: {}", code);
        joy_device(device,code);
    }
}

fn joy_device(device: uinput::Device ,code : String){
    println!("code is {}", code);
}
    


