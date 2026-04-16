use local_ip_address::local_ip;
use qrcode::QrCode;

pub fn get_local_ip() -> String {
    return local_ip().unwrap().to_string();
}

pub fn display_qr_code(content: &str) {
    let code: QrCode = qrcode::QrCode::new(content).unwrap();
    
    let string = code.render::<char>()
        .quiet_zone(true)     // Scanners NEED this white border
        .light_color(' ')     // Two spaces for light
        .dark_color('█')      // Full block for dark
        .build();

    for line in string.lines() {
        for ch in line.chars() {
            print!("{}{}", ch, ch);
        }
        println!();
    }
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