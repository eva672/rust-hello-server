use reqwest::multipart;
use reqwest::Client;
use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
};
use walkdir::WalkDir;

async fn read_file(file_path: &str) -> io::Result<(String, Vec<u8>)> {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(String::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let mut buffer = Vec::new();
    File::open(file_path)?.read_to_end(&mut buffer)?;

    Ok((file_name, buffer))
}

async fn upload_file(
    client: &Client,
    url: &str,
    file_name: String,
    file_data: Vec<u8>,
) -> Result<String, reqwest::Error> {
    let part = multipart::Part::bytes(file_data)
        .file_name(file_name.clone())
        .mime_str("application/octet-stream")
        .unwrap_or_else(|_| panic!("Failed to set MIME type")); // Should not fail in practice

    let form = multipart::Form::new().part("file", part);

    let response = client.post(url).multipart(form).send().await?;

    if response.status().is_success() {
        println!("File '{}' uploaded successfully", file_name);
        let response_body = response.text().await?;
        Ok(response_body)
    } else {
        panic!("Failed to upload file: {:?}", response.status());
    }
}

async fn process_path(path: &str) -> io::Result<Vec<(String, Vec<u8>)>> {
    let mut files = Vec::new();

    if Path::new(path).is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path().to_str().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path")
                })?;
                files.push(read_file(file_path).await?);
            }
        }
    } else {
        files.push(read_file(path).await?);
    }

    Ok(files)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path_or_directory> [...]", args[0]);
        return;
    }

    let client = Client::new();
    let url = "http://localhost:3000/uploader/upload";

    for path in &args[1..] {
        match process_path(path).await {
            Ok(files) => {
                for (file_name, file_data) in files {
                    match upload_file(&client, url, file_name, file_data).await {
                        Ok(data) => println!("{data}"),
                        Err(err) => eprintln!("Error uploading file: {}", err),
                    }
                }
            }
            Err(err) => eprintln!("Error processing path '{}': {}", path, err),
        }
    }
}