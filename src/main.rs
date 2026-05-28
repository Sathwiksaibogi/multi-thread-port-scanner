mod args;
use args::CliArgs;
use clap::Parser;

fn main() {
    let args=CliArgs::parse();

    println!("port configuration");
    println!("target : {}",args.target);
    println!("start port {} -> end port {}",args.start_port,args.end_port);
    println!("threads : {} ",args.threads);
}
