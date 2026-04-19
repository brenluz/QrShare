# qr-share

A CLI tool that transfers files from your computer to any phone — instantly, over your local network, with no apps, accounts, or cables required.

## How it works

Run the command with any file path, and a QR code appears in your terminal. Scan it with your phone's camera and the file downloads directly — no third-party services involved, everything stays on your local network.

```bash
shareQr ./example.pdf
```

## Tech

Built in Rust. Uses a raw TCP listener to serve the file over HTTP, so any phone browser can receive it without installing anything. The QR code encodes the local IP and port directly, making the connection fully peer-to-peer on the LAN.

## Installation

Requires Rust and Cargo.

```bash
git clone https://github.com/yourname/qr-share
cd qr-share
cargo install --path .
```

## Usage

```bash
shareQr <file>
```

Run from any directory — the path resolves relative to wherever you are in the terminal.