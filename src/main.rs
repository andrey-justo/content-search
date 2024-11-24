mod lib;

use lib::command_parser::SearchArgs;
use lib::file_search::search;
use clap::Parser;

fn main() {
    let args = SearchArgs::parse();
    let enabled_metrics: bool = args.enable_metrics;
    let debug_output: bool = args.debug;

    let all_files = lib::file_search::get_files_from_directory(&args.path);
    if all_files.is_empty() {
        panic!("Please select a folder with files or only a file")
    }

    let all_finds = search(args);
    if debug_output {
        for results in all_finds.0 {
            println!("Founds:{:?} in {:?} for {:?}", results.finds.unwrap_or_default().len(), results.file_name, results.search_type);
        }
    }

    let mut metrics_data = all_finds.1;
    if enabled_metrics {
        let aggregations = metrics_data.aggregate();
        for aggregation in aggregations {
            println!("Final Metrics for {:?} for {:?}", aggregation.1, aggregation.0);
        }
    }
}
