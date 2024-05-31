#![allow(clippy::get_first)]
#![allow(clippy::len_zero)]
use booktyping_core::{config::AppConfig, app::{App, AppResult, KeyPress, Test, Text}};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::panic;
use std::{fs, io};

pub mod event;
pub mod file_sys;
use event::*;

use clap::{Args, Parser, Subcommand};
use v_utils::io::ExpandedPath;

#[derive(Parser, Default)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	//todo config
	#[command(subcommand)]
	command: Commands,
	/// path to config
	#[arg(long, default_value = "~/.config/booktyping.toml")]
	config: ExpandedPath,
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
impl Default for Commands {
	fn default() -> Self {
		Self::Run(Default::default())
	}
}

#[derive(Clone, Debug, Default, Args)]
struct RunArgs {
	/// Run the web version of the app with a local copy of the book (not implemented)
	#[arg(short, long)]
	web: bool,

	/// Whether to ignore errors that are likely caused by misreading and not typing.
	#[arg(short, long)]
	myopia: bool,

	book: String,
}

fn main() {
	let cli = Cli::parse();
	let config = AppConfig::read(&cli.config.0).unwrap();
	let config = update_config(config, &cli);
	match cli.command {
		Commands::Run(args) => {
			run_app(config, args.book).unwrap();
		}
	}
}

fn run_app(config: AppConfig, book: String) -> AppResult<()> {
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

	let _ = fs::create_dir(&config.library.join(&book));

	let book_text = file_sys::load_book(&book).expect("Failed to load book");

	let tests = file_sys::load_tests(&book).expect("Failed to load tests");

	let save = move |tests: Vec<Test>, keypresses: Vec<KeyPress>| {
		file_sys::save_tests(&book, &tests)?;
		file_sys::save_keypresses(&book, &keypresses)?;
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
				app.handle_key_events(&config, key_event)?;
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

/// goes through each field, with cli-provided values overriding read config values, but only when they are different from the default
pub fn update_config(app_config: AppConfig, cli: &Cli) -> AppConfig {
	let mut out_config = app_config;
	let default_cli = Cli::default();

	if cli.library.as_ref() != default_cli.library.as_ref() {
		out_config.library = cli.library.0.clone();
	}

	match &cli.command {
		Commands::Run(args) => {
			let default_run = RunArgs::default();
			if args.myopia != default_run.myopia {
				out_config.myopia = args.myopia;
			}
		}
	}
	out_config
}
