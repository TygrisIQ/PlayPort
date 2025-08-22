use  std::net::UdpSocket;
use std::sync::{mpsc, Arc};
use std::thread;
use std::net::{TcpStream, TcpListener};
use std::io::Read;
pub fn broadcast() -> !{
   let udp_address = "0.0.0.0:8005";
   let socket = UdpSocket::bind("0.0.0.0:8005").unwrap(); 
   println!("UDP DISCOVERY ON: {}", udp_address); 

   let mut buf = [0; 1024];
   loop{ 
   let (content, source) = socket.recv_from(&mut buf).unwrap();

   let message = String::from_utf8_lossy(&buf[..content]);
    
   if message.trim() == "IamClient" {
       println!("UDP MESSAGE RECEIVED: {}", message.trim());
       let reply = "IamServer";
       socket.send_to(reply.as_bytes(), source).unwrap();
   }
}}

fn handle_client(mut stream: TcpStream) -> String{  
        let mut buf = [0; 1024];
       loop{  
        match stream.read(&mut buf){
            Ok(0) => {
                println!("connection closed!");
                break String::new();
            }
            Ok(br) =>{
                  let received = String::from_utf8_lossy(&buf[..br]);
                  println!("Received: {}", received);
                  return received.to_string();       
            }
            Err(e) => {
                eprintln!("Tcp stream parse error: {}",e);
            }
        }

       }
    

}
pub fn tcp_listener(port: &String, tx: mpsc::Sender<String>){
    let ip = "0.0.0.0".to_owned() + ":" +&port;
    println!("{}",ip);
    let listener = TcpListener::bind(ip).expect("could not bind to Ip and port");
    println!("local addr: {}",listener.local_addr().unwrap()); 
    //
    //Listen to stream
    for stream in listener.incoming(){
        //hanlde incoming stream
        match stream{
            Ok(stream) => {
                println!("TCP LISTENER RUNNING..");
                let result = handle_client(stream);
                tx.send(result).unwrap();
            },
            Err(e) => {
                eprint!("{}",e);
            }
        }
        } 
}
