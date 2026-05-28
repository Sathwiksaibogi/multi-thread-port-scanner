use clap::Parser;

#[derive(Parser,Debug)]
// This tells clap to read the entire struct layout and automatically implement a hidden trait named Parser. This trait generates the actual state machine required to scan through terminal strings.
#[command(author,version,about="A high-performance multi-threaded port scanner")]

pub struct CliArgs{
    #[arg(short,long)]
    // This tells the code generator: "Hey, create command-line flags for this specific field." ---
    pub target : String,

    #[arg(short, long, default_value_t = 1)]
    pub start_port: u16,

    #[arg(short, long, default_value_t = 1024)]
    pub end_port: u16,

    #[arg(long, default_value_t = 4)]
    pub threads: u32,

}