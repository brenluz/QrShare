use std::path::PathBuf;
use clap::Parser;
use std::fs;
use std::io::{Read, Write};

mod utils;
use shareQr::{self};

#[derive(Parser)]
struct CLi {
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = CLi::parse();

    // Verification
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

    let (listener, content) = shareQr::setup_listener(utils::get_local_ip());

    println!("Scan the QR code to download the file:");
    utils::display_qr_code(&content);
    println!("Waiting for phone to scan and connect...");

    let file_name = args.path.file_name().unwrap().to_string_lossy();
    let file_size = file_data.len();
    let mut transferred = false;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let addr = stream.peer_addr().unwrap();
                println!("Connection from {}", addr);

                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf);

                let header = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: application/octet-stream\r\n\
                    Content-Length: {}\r\n\
                    Content-Disposition: attachment; filename=\"{}\"\r\n\
                    Connection: close\r\n\r\n",
                    file_size, file_name
                );

                stream.write_all(header.as_bytes()).expect("Failed to send header");

                match std::io::copy(&mut &file_data[..], &mut stream) {
                    Ok(bytes) => {
                        stream.flush().ok();
                        println!("Sent {} bytes — done!", bytes);
                        transferred = true;
                    }
                    Err(e) => {
                        eprintln!("Incomplete transfer: {} — retrying...", e);
                    }
                }
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }

        if transferred {
            break;
        }
    }
    Ok(())
}