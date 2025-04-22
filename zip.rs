use std::fs::File;
use std::io::{Read, Write, Cursor};
use zip::write::FileOptions;

/// `files` is a vector of tuples: (actual file path, name inside zip)
pub fn zip_files(files: Vec<(String, String)>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut buffer);

    for (path, zip_name) in files {
        let mut f = File::open(path)?;
        let mut contents = Vec::new();
        f.read_to_end(&mut contents)?;

        zip.start_file(zip_name, FileOptions::default())?;
        zip.write_all(&contents)?;
    }

    zip.finish()?;
    Ok(buffer.into_inner())
}



use std::io::{Cursor, Write};
use zip::write::FileOptions;

/// Zips in-memory files into a Vec<u8> ZIP archive
pub fn zip_in_memory_files(
    files: Vec<(String, Vec<u8>)>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut buffer);

    for (filename, content) in files {
        zip.start_file(filename, FileOptions::default())?;
        zip.write_all(&content)?;
    }

    zip.finish()?;
    Ok(buffer.into_inner())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec![
        ("hello.txt".to_string(), b"Hello, world!".to_vec()),
        ("folder/readme.md".to_string(), b"This is a test.".to_vec()),
    ];

    let zip_bytes = zip_in_memory_files(files)?;

    std::fs::write("in_memory_output.zip", zip_bytes)?;
    println!("In-memory ZIP created as in_memory_output.zip");

    Ok(())
}