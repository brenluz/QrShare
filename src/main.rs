use std::path::PathBuf;
use clap::Parser;
use qrcode::QrCode;
use std::fs;

mod utils;
use qr_share::*;

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

    let content = qr_share::start_server(utils::get_local_ip());
    let listener = qr_share::listen_on_port(content.split(':').last().unwrap().parse::<u16>().unwrap());

    let code = QrCode::new(content).unwrap();

    let string = code.render()
    .light_color(' ')
    .dark_color('#')
    .build();
    println!("{}", string);

    println!("Local IP: {}", utils::get_local_ip());
}
