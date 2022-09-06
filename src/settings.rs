use clap::Parser;

#[derive(Parser, Debug)]
pub struct Settings {
    #[clap(short, long, value_parser, default_value = "1")]
    pub interval: f64,
    #[clap(short, long, value_parser, default_value = "10")]
    pub width: u32,
    #[clap(short, long, value_parser, default_value = "20")]
    pub height: u32,
}
