pub fn parse_ports(input: &str) -> Result<Vec<u16>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Port input cannot be empty".to_string());
    }

    let mut ports = Vec::new();

    for part in trimmed.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return Err("Invalid port specification: empty segment found".to_string());
        }

        if part.contains('-') {
            let segments: Vec<&str> = part.split('-').collect();
            if segments.len() != 2 {
                return Err(format!("Invalid port range format: '{}'", part));
            }

            let start = segments[0]
                .trim()
                .parse::<u16>()
                .map_err(|_| format!("Invalid start port in range: '{}'", segments[0]))?;

            let end = segments[1]
                .trim()
                .parse::<u16>()
                .map_err(|_| format!("Invalid end port in range: '{}'", segments[1]))?;

            if start == 0 || end == 0 {
                return Err("Port number must be between 1 and 65535".to_string());
            }

            if start > end {
                return Err(format!(
                    "Invalid port range: start port {} is greater than end port {}",
                    start, end
                ));
            }

            for port in start..=end {
                ports.push(port);
            }
        } else {
            let port = part
                .parse::<u16>()
                .map_err(|_| format!("Invalid port number: '{}'", part))?;

            if port == 0 {
                return Err("Port number must be between 1 and 65535".to_string());
            }

            ports.push(port);
        }
    }

    ports.sort_unstable();
    ports.dedup();

    if ports.is_empty() {
        return Err("No valid ports specified".to_string());
    }

    Ok(ports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_port() {
        assert_eq!(parse_ports("80").unwrap(), vec![80]);
    }

    #[test]
    fn test_multiple_ports() {
        assert_eq!(parse_ports("22,80,443").unwrap(), vec![22, 80, 443]);
    }

    #[test]
    fn test_port_range() {
        assert_eq!(parse_ports("1-5").unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_combined_ports() {
        assert_eq!(parse_ports("22,80-82,443").unwrap(), vec![22, 80, 81, 82, 443]);
    }

    #[test]
    fn test_invalid_ports() {
        assert!(parse_ports("").is_err());
        assert!(parse_ports("abc").is_err());
        assert!(parse_ports("100-50").is_err());
        assert!(parse_ports("0").is_err());
        assert!(parse_ports("70000").is_err());
    }
}
