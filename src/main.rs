#![allow(clippy::get_first)]
#![allow(clippy::len_zero)]
use clap::{Args, Parser, Subcommand};
use v_utils::io::ExpandedPath;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
	/// Where the books are stored
	#[arg(long, default_value = "~/.booktyping")]
	library: ExpandedPath,
}
#[derive(Subcommand)]
enum Commands {
	/// Starts the thing
	///Ex
	///```sh
	///booktyping run -w "!"
	///```
	Run(RunArgs),
}

#[derive(Clone, Debug, Default, derive_new::new, Args)]
struct RunArgs {
	/// Hello to world
	#[arg(short, long)]
	web: bool,

	book: String,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Run(args) => {
			dbg!(&args);
		}
	}
}
