use gul_repl; 
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/dev/ttyUSB0")]
    port: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args.port)
}
