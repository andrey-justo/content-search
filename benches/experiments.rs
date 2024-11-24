#[cfg(test)]
mod tests {
    use file_search::command_parser::SearchArgs;
    use file_search::file_search::search;

    fn search_with_metrics(test_name: &str, args: SearchArgs) {
        let mut found = search(args);
        let aggregations = found.1.aggregate();
        println!("Loading Metrics..");
        for aggregation in aggregations {
            println!("{:?}: {:?} - {:?} - ", test_name, aggregation.0, aggregation.1, );
        }
    }

    #[test]
    fn test_experiment_with_1000_files() {
        let path = std::env::current_dir();
        println!("The current directory is {}", path.unwrap().display());

        let rabin_args = SearchArgs {
            debug: false,
            enable_metrics: true,
            path: PathBuf::from("./experiments/batch_files"),
            search_pattern: String::from("estibulum"),
            search_type: SearchType::Rabin
        };

        search_with_metrics("Rabin 1000 files", rabin_args.clone());

        let mut booyer_args = rabin_args.clone();
        booyer_args.search_type = SearchType::Booyer;

        search_with_metrics("Booyer 1000 files", booyer_args.clone());
    }

    #[test]
    fn test_experiment_with_big_file() {
        let rabin_args = SearchArgs {
            debug: false,
            enable_metrics: true,
            path: PathBuf::from("./experiments/larger_files"),
            search_pattern: String::from("massa"),
            search_type: SearchType::Rabin
        };

        search_with_metrics("Rabin Big File", rabin_args.clone());

        let mut booyer_args = rabin_args.clone();
        booyer_args.search_type = SearchType::Booyer;

        search_with_metrics("Booyer Big File", booyer_args.clone());
    }
}
