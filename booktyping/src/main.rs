#![allow(clippy::get_first)]
#![allow(clippy::len_zero)]
use booktyping_core::app::{App, AppResult, KeyPress, Test, Text};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::panic;
use std::path::Path;
use std::{fs, io};

pub mod event;
pub mod file_sys;
use event::*;

use clap::{Args, Parser, Subcommand};
use v_utils::io::ExpandedPath;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
	/// Where the books are stored
	#[arg(long, default_value = "~/.booktyping")]
	//NB: `ExpandedPath` may or may not break on windows, but who cares
	library: ExpandedPath,
}
#[derive(Subcommand)]
enum Commands {
	/// Run
	///Ex:
	///```sh
	///booktyping run
	///```
	Run(RunArgs),
}

#[derive(Clone, Debug, Default, Args)]
struct RunArgs {
	/// Run the web version of the app with a local copy of the book (not implemented)
	#[arg(short, long)]
	web: bool,

	book: String,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Run(args) => {
			run_app(args, cli.library.as_ref()).unwrap();
		}
	}
}

fn run_app(args: RunArgs, library: &Path) -> AppResult<()> {
	let backend = CrosstermBackend::new(io::stderr());
	let mut terminal = Terminal::new(backend)?;

	crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
	terminal::enable_raw_mode()?;

	let panic_hook = panic::take_hook();
	panic::set_hook(Box::new(move |panic| {
		let _ = terminal::disable_raw_mode();
		let _ = crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture);
		panic_hook(panic);
	}));

	let _ = fs::create_dir(library.join(&args.book));

	let book_text = file_sys::load_book(&args.book).expect("Failed to load book");

	let tests = file_sys::load_tests(&args.book).expect("Failed to load tests");

	let save = move |tests: Vec<Test>, keypresses: Vec<KeyPress>| {
		file_sys::save_tests(&args.book, &tests)?;
		file_sys::save_keypresses(&args.book, &keypresses)?;
		Ok(())
	};

	let text = Text::new(book_text, tests, save, 0);
	let mut app = App::new(terminal.size()?.width, text);

	let events = EventHandler::new(250);

	terminal.hide_cursor()?;
	terminal.clear()?;
	terminal.draw(|frame| app.render(frame))?;

	while app.running {
		match events.next()? {
			Event::Key(key_event) => {
				app.handle_key_events(key_event)?;
			}
			Event::Resize(width, _) => {
				app.terminal_width = width;
				app.generate_lines();
			}
		}
		terminal.draw(|frame| app.render(frame))?;
	}

	terminal::disable_raw_mode()?;
	crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
	terminal.show_cursor()?;
	Ok(())
}
