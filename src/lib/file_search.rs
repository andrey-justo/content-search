use std::{fs, path::PathBuf};

use super::{booyer_moore_search, command_parser::{SearchArgs, SearchType}, metrics::{self, MetricData}, rabin_karpin_search};

pub struct FileContentResult {
    pub file_name: String,
    pub finds: Option<Vec<usize>>,
    pub search_type: SearchType
}

pub fn search(args: SearchArgs) -> (Vec<FileContentResult>, MetricData) {
    let all_files = get_files_from_directory(&args.path);
    if all_files.is_empty() {
        panic!("Please select a folder with files or only a file")
    }

    return search_buffered_files(args, all_files);
}

pub fn search_buffered_files(args: SearchArgs, all_files: Vec<PathBuf>) -> (Vec<FileContentResult>, MetricData) {
    let mut metrics_data = metrics::MetricData{ points: None };
    let mut all_finds: Vec<FileContentResult> = Vec::new();
    for file in all_files.iter() {
        let content = fs::read_to_string(file);
        if content.is_err() {
            panic!("Couldn't read file {:?}", file);
        }

        let c = content.expect("No Content available");
        let file_path = file.to_str().unwrap();
        if args.search_type == SearchType::Both {
            let found_index = metrics_data.add_new_metric(file_path, c.as_str(), &args.search_pattern, SearchType::Boyer, search_by_file);
            all_finds.push(FileContentResult{
                file_name: String::from(file_path),
                finds: found_index.0,
                search_type: SearchType::Boyer
            });

            let found_index_rabin = metrics_data.add_new_metric(String::from(file.to_str().unwrap()).as_mut_str(), c.as_str(), &args.search_pattern, SearchType::Rabin, search_by_file);
            all_finds.push(FileContentResult{
                file_name: String::from(file_path),
                finds: found_index_rabin.0,
                search_type: SearchType::Rabin
            });
        } else {
            let found_index = metrics_data.add_new_metric(String::from(file.to_str().unwrap()).as_mut_str(), c.as_str(), &args.search_pattern, args.search_type.clone(), search_by_file);
            all_finds.push(FileContentResult{
                file_name: String::from(file_path),
                finds: found_index.0,
                search_type: args.search_type
            });
        }
    }

    return (all_finds, metrics_data);
}

pub fn search_by_file(content: &str, pattern: &str, s_type: SearchType) -> Option<Vec<usize>> {
    if s_type == SearchType::Boyer {        
        return Some(booyer_moore_search::boyer_moore_search(&content, &pattern));
    } else if s_type == SearchType::Rabin {
        return Some(rabin_karpin_search::rabin_karpin_search(&content, &pattern));
    }

    return None;
}

pub fn get_files_from_directory(path: &PathBuf) -> Vec<PathBuf> {
    let is_directory = path.metadata().expect("Please select a valid OS Path").is_dir();
    if !is_directory {
        return vec![path.to_path_buf()];
    }

    let mut all_files: Vec<PathBuf> = Vec::new();
    for path in fs::read_dir(path).unwrap() {
        if path.is_err() {
            panic!("Couldn't read file {:?}", path);
        }

        let result_path = path.unwrap();
        if result_path.metadata().unwrap().is_dir() {
            all_files.append(&mut get_files_from_directory(&result_path.path()));
        } else {
            all_files.push(result_path.path());
        }
    }
    return all_files;
}
