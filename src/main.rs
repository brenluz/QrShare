use std::path::PathBuf;
use clap::Parser;
use std::fs;
use std::io::Write;
mod utils;
use qr_share::{self};

#[derive(Parser)]
struct CLi {
    path: PathBuf,
}

fn main() {
    let args = CLi::parse();

    if !args.path.exists() {
        eprintln!("Error: File does not exist.");
        std::process::exit(1);
    }

    if !args.path.is_file() {
        eprintln!("Error: Path is not a file.");
        std::process::exit(1);
    }

    let file_data = fs::read(&args.path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        std::process::exit(1);
    });

    let (listener, content) = qr_share::setup_listener(utils::get_local_ip());

    println!("Scan the QR code to download the file:");
    utils::display_qr_code(&content);
    println!("Waiting for phone to scan and connect...");

    let file_name = args.path.file_name().unwrap().to_string_lossy();
    let file_size = file_data.len();

    match listener.accept() {
        Ok((mut stream, addr)) => {
            
           let header = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: {}\r\n\
             Content-Disposition: attachment; filename=\"{}\"\r\n\
             Connection: close\r\n\r\n",
            file_size, file_name
        );

            stream.write_all(header.as_bytes()).expect("Failed to send header");

            println!("Sending file data...");
            std::io::copy(&mut &file_data[..], &mut stream).expect("Failed to send file");

            stream.flush().expect("Failed to flush");
            println!("File sent successfully!");

        }

        
        Err(e) => {
            eprintln!("Failed to accept connection: {}", e);
        }
    }
}
