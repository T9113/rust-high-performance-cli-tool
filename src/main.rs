use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fastcli", version = "1.0", about = "Blazing fast Rust CLI")]
struct Args {
    #[arg(short, long)]
    target: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Executing high-speed operations on: {}", args.target);
}
