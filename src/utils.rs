use local_ip_address::local_ip;

pub fn get_local_ip() -> String {
    return local_ip().unwrap().to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        let ip = get_local_ip();
        assert!(!ip.is_empty());
    }
}