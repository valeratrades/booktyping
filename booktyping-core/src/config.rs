use std::path::{Path, PathBuf};

use anyhow::Result;
use v_utils::macros::MyConfigPrimitives;

/// AppConfig is the sentral settings of the entire program.
#[derive(Clone, Debug, Default, MyConfigPrimitives)]
pub struct AppConfig {
	pub myopia: bool,
	pub library: PathBuf,
}

impl AppConfig {
	/// NB: if file does not exist, we return default instead of erroring
	pub fn read(path: &Path) -> Result<Self> {
		let settings = {
			if path.exists() {
				let builder = config::Config::builder().add_source(config::File::with_name(path.to_str().unwrap()));
				let raw: config::Config = builder.build()?;
				raw.try_deserialize()?
			} else {
				AppConfig::default()
			}
		};

		Ok(settings)
	}
}
