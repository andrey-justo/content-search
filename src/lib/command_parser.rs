use clap::{Parser, ValueEnum};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct SearchArgs {
    /// The pattern to look for
    #[arg(long)]
    pub search_pattern: String,
    /// The path to the file to read
    #[arg(long)]
    pub path: std::path::PathBuf,
    // Search type
    #[arg(long)]
    pub search_type: SearchType,
    // Calculate Metrics for Benchmark
    #[arg(short, long)]
    pub enable_metrics: bool,
    // Output to show finds
    #[arg(short, long)]
    pub debug: bool
}

// Use Search type for comparing algorithms
#[derive(Copy, ValueEnum, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchType {
    Rabin = 0,
    Boyer = 1,
    Both = 2,
}