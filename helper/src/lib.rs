use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use serde::de::DeserializeOwned;
use serde_jsonlines::json_lines;
use std::path::Path;

pub mod prelude {
    pub use anyhow::{Context, Result};
    pub use indicatif::{ProgressBar, ProgressStyle};
    pub use itertools::Itertools;
    pub use rayon::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_jsonlines::json_lines;
    pub use std::collections::{HashMap, HashSet};
}

/// Load JSONL data from a file into a vector of type T
pub fn load_jsonl<T>(path: impl AsRef<Path>) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    Ok(json_lines(path)?.collect::<std::io::Result<Vec<T>>>()?)
}

/// Create a progress bar with a specific style
pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

/// Helper function to print solutions
pub fn print_solution(candle_num: u32, solution: impl std::fmt::Display) {
    println!("🕯️ Candle {candle_num}: {solution}");
}
