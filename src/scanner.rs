use std::net::{TcpStream,ToSocketAddrs};
use std::time::Duration;

pub fn scan_port(target:&str,port:u16)->bool{
    let address=format!("{}:{}",target,port);

    // match address.to_socket_addrs(){
    //     // address.to_socket_addrs() provides Result<(),e>
    //     Ok(mut addr)=>{
    //         // mut addr => becomes .next() provides the first item in the list , if called second time it will give the same item if not mutated
    //         match addr.next(){
    //             // addr.next gives you option<_,_>

    //             Some(socket_addr)=>{
    //                 match TcpStream::connect_timeout(&socket_addr,Duration::from_millis(1500)){
    //                     Ok(_)=>{
    //                         true
    //                     }
    //                     Err(_)=>{
    //                         false
    //                     }
    //                 }

    //             }
    //             None=>{
    //                 false
    //             }

    //         }

    //     }
    //     Err(_)=>{
    //         false

    //     }
    // }

    if let Ok(mut addr)=address.to_socket_addrs(){
        if let Some(socket_addr)=addr.next(){
            if let Ok(_)=TcpStream::connect_timeout(&socket_addr, Duration::from_millis(1500)){
                return true;
            }
        }
    }

    false


}