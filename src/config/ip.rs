
pub fn is_valid_ip(ip_adress: &str) -> bool {
    let octets: Vec<&str> = 
        ip_adress.split(".").collect();
    
    if octets.len() != 4 {
        return false;
    }

    for octet in octets {
        match octet.parse::<u32>() {
            Ok(num) => {
                if num > 255 {
                    return false;
                }
            },
            Err(_) => {
                return false;
            }
        }
    }
    true
}