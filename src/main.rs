mod configuration;
mod scanner;
use configuration::CliArgs;
use scanner::scan_port;
use clap::Parser;
use std::net::{TcpStream,ToSocketAddrs};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn main() {
    let mut args=CliArgs::parse();


    let (tx,rx)=mpsc::channel();

    let mut handles=vec![];

    println!("scanning target {}",args.target);


    for thread_ind in 0..args.threads{
        let thread_tx=tx.clone();
        let target=args.target.clone();
        let start=args.start_port+ thread_ind as u16;

        let handle=thread::spawn(move || {
            for port in (start..args.end_port+1).step_by(args.threads as usize){
            if scanner::scan_port(&target, port){
                thread_tx.send(port);
            }
        } 
        });
        handles.push(handle);
    }
    drop(tx);

    for open_port in rx{
        println!("found open port {}",open_port);
    }

    for handle in handles{
        let th=handle.join();
    }

    println!("scan completed");
    // println!("port configuration");
    // println!("target : {}",args.target);
    // println!("start port {} -> end port {}",args.start_port,args.end_port);
    // println!("threads : {} ",args.threads);

//     let status_80=scanner::scan_port("google.com", 80);
//     println!("result :{}",status_80);
}
