use std::env;
use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::path::Path;
use zip::write::FileOptions;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    let path = Path::new(dir_path);

    if !path.is_dir() {
        eprintln!("The provided path is not a directory");
        std::process::exit(1);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            zip_folder(&entry_path)?;
        }
    }

    println!("All folders have been converted to zip files.");
    Ok(())
}

use zip::write::ZipWriter;
use zip::CompressionMethod;

fn zip_folder(folder_path: &Path) -> std::io::Result<()> {
    let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
    let zip_file_name = format!("{}.zip", folder_name);
    let zip_path = folder_path.with_file_name(&zip_file_name);

    let file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);

    let options =
        FileOptions::default().compression_method(CompressionMethod::Stored);

    add_folder_to_zip(&mut zip, folder_path, folder_path, &options)?;

    zip.finish()?;
    println!("Created zip file: {:?}", zip_path);
    Ok(())
}

fn add_folder_to_zip<T: Write + Seek>(
    zip: &mut ZipWriter<T>,
    base_path: &Path,
    folder_path: &Path,
    options: &FileOptions,
) -> std::io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(base_path).unwrap();

        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), *options)?;
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() {
            zip.add_directory(name.to_str().unwrap(), *options)?;
            add_folder_to_zip(zip, base_path, &path, options)?;
        }
    }
    Ok(())
}
