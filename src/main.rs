mod configuration;
mod scanner;
use configuration::CliArgs;
use scanner::scan_port;
use clap::Parser;
use std::net::{TcpStream,ToSocketAddrs};
use std::time::Duration;

fn main() {
    let args=CliArgs::parse();

    for thread_ind in 0..args.threads{
        let start=args.start_port+ thread_ind as u16;

        for port in (start..args.end_port+1).step_by(args.threads as usize){
            println!("thread_ind : {}, port :{}",thread_ind,port);

        } 
        println!();

    }

    // println!("port configuration");
    // println!("target : {}",args.target);
    // println!("start port {} -> end port {}",args.start_port,args.end_port);
    // println!("threads : {} ",args.threads);

//     let status_80=scanner::scan_port("google.com", 80);
//     println!("result :{}",status_80);
}
