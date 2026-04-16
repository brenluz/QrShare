use std::net::TcpListener;

// We return both the listener AND the string for the QR code
pub fn setup_listener(local_ip: String) -> (TcpListener, String) {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind");
    
    let port = listener.local_addr().expect("Failed to get addr").port();

    let content = format!("http://{}:{}/download", local_ip, port);
    
    (listener, content)
}