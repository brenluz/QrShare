use std::net::TcpListener;

pub fn find_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
    let port = listener.local_addr().expect("Failed to get local address").port();

    return port;
}

pub fn listen_on_port(port: u16) -> TcpListener {
    return TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind to port");
}

pub fn start_server(local_ip: String) -> String {
    let port = find_available_port();

    println!("Server started on port {}", port);
    println!("Local IP: {}", local_ip);

    let content = format!("http://{}:{}/download", local_ip, port);
    println!("Content: {}", content);

    return content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_port() {
        let port = find_available_port();
        assert!(port > 0);
    }

    #[test]
    fn test_listen_on_port() {
        let port = find_available_port();
        let listener = listen_on_port(port);
        assert!(listener.local_addr().unwrap().port() == port);
    }
}