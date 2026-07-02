use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum AnswerType {
    Pretied,
    Raw,
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "CLI utility to extract color palettes from images")]
pub struct Args {
    /// Path to image file
    #[arg(short, long)]
    pub image: PathBuf,

    /// Number of colors to extract
    #[arg(short, long, default_value_t = 5)]
    pub count: usize,

    /// Type of answer to display
    #[arg(short, long, value_enum, default_value = "pretied")]
    pub answer_type: AnswerType,

    /// Show debug information and logs
    #[arg(short, long)]
    pub debug: bool,
}