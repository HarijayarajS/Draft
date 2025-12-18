use std::fs;

fn count_rs_files(path: &str) -> std::io::Result<usize> {
    Ok(fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .count())
}