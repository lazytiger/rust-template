use clap::Parser;

use crate::options::Options;

mod options;
mod types;
mod logger;

pub fn run() ->types::Result<()> {
    let options = Options::parse();
    logger::setup_logger(options.log_file.as_str(), options.log_level)?;
    Ok(())
}
