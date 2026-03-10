use std::net::{UdpSocket, TcpStream, TcpListener};
use std::sync::mpsc;
use std::thread;
use std::io::{BufRead, BufReader};
pub fn broadcast() -> !{
   
   let socket = UdpSocket::bind("0.0.0.0:8005").unwrap(); 
//    println!("UDP DISCOVERY ON: {}", udp_address); 

   let mut buf = [0; 1024];
   loop{ 
    let (len, source) = socket.recv_from(&mut buf).unwrap();
        let message = String::from_utf8_lossy(&buf[..len]);


        if message.trim() == "IamClient" {
            println!("UDP: client found at {}", source);
            socket.send_to(b"IamServer", source).unwrap();
        }
}}


fn handle_client(stream: TcpStream, tx: mpsc::Sender<String>) {
    let peer = stream.peer_addr().ok();
    println!("Client connected: {:?}", peer);
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(msg) if !msg.trim().is_empty() => {
                if tx.send(msg).is_err() {
                    break; 
                }
            }
            //blank line => skip
            Ok(_) => {} 
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
    println!("client disconnected!! {:?}", peer);
}
// fn handle_client(mut stream: TcpStream) -> String{  
//         let mut buf = [0; 1024];
//        loop{  
//         match stream.read(&mut buf){
//             Ok(0) => {
//                 println!("connection closed!");
//                 break String::new();
//             }
//             Ok(br) =>{
//                   let received = String::from_utf8_lossy(&buf[..br]);
//                   println!("Received: {}", received);
//                   return received.to_string();       
//             }
//             Err(e) => {
//                 eprintln!("Tcp stream parse error: {}",e);
//             }
//         }

//        }
    

// }
pub fn tcp_listener(port: &String, tx: mpsc::Sender<String>){
    let ip = "0.0.0.0".to_owned() + ":" +&port;
    println!("{}",ip);
    let listener = TcpListener::bind(ip).expect("could not bind to Ip and port");
    println!("local addr: {}",listener.local_addr().unwrap()); 
    println!("TCP listening on {}", listener.local_addr().unwrap());
    //
    //Listen to stream
    for stream in listener.incoming(){
        //hanlde incoming stream
        match stream{
            Ok(stream) => {
                let tx = tx.clone();
                thread::spawn(move || handle_client(stream, tx));
            },
            Err(e) => {
                eprint!("{}",e);
            }
        }
        } 
}
