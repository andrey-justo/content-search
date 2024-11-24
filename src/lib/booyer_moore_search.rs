use std::cmp::max;
use std::collections::HashMap;

fn build_bad_char_table(pat: &[char]) -> HashMap<char, isize> {
    let mut bad_char_table = HashMap::new();
    for (i, &ch) in pat.iter().enumerate() {
        bad_char_table.insert(ch, i as isize);
    }
    bad_char_table
}

pub fn boyer_moore_search(text: &str, pat: &str) -> Vec<usize> {
    let mut positions = Vec::new();

    let text_len = text.len() as isize;
    let pat_len = pat.len() as isize;

    // Handle edge cases where the text or pattern is empty, or the pattern is longer than the text
    if text_len == 0 || pat_len == 0 || pat_len > text_len {
        return positions;
    }

    // Convert text and pattern to character vectors for easier indexing
    let pat: Vec<char> = pat.chars().collect();
    let text: Vec<char> = text.chars().collect();

    // Build the bad character table for the pattern
    let bad_char_table = build_bad_char_table(&pat);

    let mut shift = 0;

    // Main loop: shift the pattern over the text
    while shift <= text_len - pat_len {
        let mut mis_idx = pat_len - 1;

        // Compare pattern from right to left
        while mis_idx >= 0 && pat[mis_idx as usize] == text[(shift + mis_idx) as usize] {
            mis_idx -= 1;
        }

        // If we found a match (j < 0), record the position
        if mis_idx < 0 {
            positions.push(shift as usize);
            let mut idx = 1;
            if shift + pat_len < text_len {
                let next_ch = text[(shift + pat_len) as usize];
                idx = pat_len - bad_char_table.get(&next_ch).unwrap_or(&-1);
            }
            
            shift += idx;
        } else {
            let mis_char = text[(shift + mis_idx) as usize];
            let bad_char_shift = bad_char_table.get(&mis_char).unwrap_or(&-1);
            let idx = max(1, mis_idx - bad_char_shift);
            shift += idx;
        }
    }

    positions
}