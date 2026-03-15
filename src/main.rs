use std::thread;
use std::sync::mpsc;
use input::JoyDevice;
mod server;
mod input;
mod debug_log;

fn main(){
    let local_ip = get_local_ip().unwrap_or_else(|| "unknown ip".to_string());
    
    
    let port= std::env::args().nth(1).unwrap_or(String::from("8007"));
    debug_log!("port :{}", port);
    //device init
    let mut jdevice = input::JoyDevice::new();
    //network
    println!(
        "PLAYPORT \n  
         ip address :{local_ip}:{port} \n
        YOU WILL NEED THE ANDROID CLIENT TO SEND INPUT \n 
        https://github.com/TygrisIQ/PlayPort_Client 
        "
    );
    //network thread
    thread::spawn(||{server::broadcast()});
    let (tx, rx) = mpsc::channel();
    let port_clone = port.clone();
    thread::spawn(move || { 
    server::tcp_listener(&port_clone, tx);    
    
    });
   
    loop {
    if let Ok(x) = rx.recv(){
       JoyDevice::handle_input(&mut jdevice, x); 
    }
    }
    }

fn get_local_ip() -> Option<String>{
    
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0" ).ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}




    


