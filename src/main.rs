#![allow(clippy::get_first)]
#![allow(clippy::len_zero)]
use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}
#[derive(Subcommand)]
enum Commands {
	/// Hello the world or rust (flags are mutually exclusive)
	///Ex
	///```sh
	///what hello -w "!"
	///```
	Hello(HelloArgs),
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("channel")
        .required(true)
        .args(&["world", "rust"]),
))]
struct HelloArgs {
	/// Hello to world
	#[arg(short, long)]
	world: bool,
	/// Hello to rust
	#[arg(short, long)]
	rust: bool,

	/// Message to send after hello
	after_hello_message: Vec<String>,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Hello(args) => {
			let hello_target = match (args.world, args.rust) {
				(true, false) => "World",
				(false, true) => "Rust",
				(true, true) => panic!("Cannot hello two things"),
				(false, false) => panic!("Specify what to hello"),
			};

			let message = format!("Hello, {hello_target}{}", &args.after_hello_message.join(""));
			println!("{message}");
		}
	}
}
