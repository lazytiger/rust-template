use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {

    /// file path for save log, empty means stdout
    #[arg(short = 'L', long, default_value = "")]
    pub log_file:String,

    /// log level, 0 to 4 represents TRACE, DEBUG, INFO, WARN, ERROR, others mean OFF
    #[arg(short = 'E', long, default_value = "2")]
    pub log_level:u8,
}