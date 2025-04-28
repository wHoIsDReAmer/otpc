use std::path::Path;

use anyhow::Result;
use rqrr::PreparedImage;
use crate::config::Account;

pub(crate) fn parse_qr_from_image(file: &Path) -> Result<Account> {
    let img = image::open(file)?;
    let mut img = PreparedImage::prepare(img.to_luma8());
    let grids = img.detect_grids();

    let qr_uri = grids[0].decode()
        .map_err(|_| anyhow::anyhow!("Failed to decode QR code"))?;
    parse_key_uri(&qr_uri.1)
}

// Key uri format: otpauth://TYPE/LABEL?PARAMETERS
pub(crate) fn parse_key_uri(uri: &str) -> Result<Account> {
    if !uri.starts_with("otpauth://") {
        return Err(anyhow::anyhow!("Invalid OTP URI scheme"));
    }

    let remaining = &uri[10..];

    let type_end = remaining.find('/').ok_or(anyhow::anyhow!("Invalid URI: Missing type separator"))?;
    let remaining = &remaining[type_end + 1..];

    // Splitting label and parameters
    let (label_part, params_part) = match remaining.find('?') {
        Some(idx) => (&remaining[..idx], Some(&remaining[idx + 1..])),
        None => (remaining, None),
    };

    // Parsing label (includes URL decoding)
    // Format: "Issuer:AccountName" or "AccountName"
    let decoded_label = url_decode(label_part)?;
    let (mut issuer, name) = match decoded_label.find(':') {
        Some(idx) => {
            // Remove spaces
            let potential_issuer = decoded_label[..idx].trim();
            let potential_name = decoded_label[idx + 1..].trim();
            // Handle empty issuer (" :AccountName")
            if potential_issuer.is_empty() {
                ("host".to_string(), potential_name.to_string())
            } else {
                (potential_issuer.to_string(), potential_name.to_string())
            }
        }
        None => ("host".to_string(), decoded_label.trim().to_string()),
    };

    if name.is_empty() {
        return Err(anyhow::anyhow!("Invalid URI: Account name cannot be empty"));
    }

    // Parsing parameters
    let mut secret = None;
    if let Some(params_str) = params_part {
        for param in params_str.split('&') {
            let pair: Vec<&str> = param.splitn(2, '=').collect();
            if pair.len() == 2 {
                let key = url_decode(pair[0])?;
                let value = url_decode(pair[1])?;

                match key.to_lowercase().as_str() {
                    "secret" => secret = Some(value),
                    "issuer" => issuer = value, // issuer parameter has higher priority
                    // other parameters (algorithm, digits, period, counter) are currently ignored
                    _ => {},
                }
            }
        }
    }

    let secret = secret.ok_or(anyhow::anyhow!("Invalid URI: Missing 'secret' parameter"))?;
    if secret.is_empty() {
        return Err(anyhow::anyhow!("Invalid URI: Secret cannot be empty"));
    }

    Ok(Account {
        name,
        secret,
        issuer,
    })
}

fn url_decode(input: &str) -> Result<String> {
    let mut result = Vec::new();
    let mut chars = input.bytes().peekable();

    while let Some(byte) = chars.next() {
        match byte {
            b'%' => {
                let h1 = chars.next().ok_or(anyhow::anyhow!("Invalid percent encoding"))?;
                let h2 = chars.next().ok_or(anyhow::anyhow!("Invalid percent encoding"))?;

                // convert hex to byte
                let hex = format!("{}{}", h1 as char, h2 as char);
                let decoded_byte = u8::from_str_radix(&hex, 16)
                    .map_err(|_| anyhow::anyhow!("Invalid hex sequence in percent encoding"))?;
                result.push(decoded_byte);
            }
            b'+' => result.push(b' '),
            _ => result.push(byte),
        }
    }

    String::from_utf8(result)
        .map_err(|e| anyhow::anyhow!("Decoded string is not valid UTF-8: {}", e))
}

#[allow(unused)]
mod tests {
    use rqrr::PreparedImage;

    use crate::qrcode::parse_key_uri;

    #[test]
    fn test_parse_qr_from_image() {
        let img = image::open("./src/assets/example.png").expect("Failed to open image");
        let mut img = PreparedImage::prepare(img.to_luma8());
        let grids = img.detect_grids();

        assert!(grids.len() > 0);

        // decode the grids
        let qr = grids[0].decode();
        println!("{:?}", qr);
    }

    #[test]
    fn test_parse_key_uri() {
        let uri = "otpauth://totp/Example:alice@google.com?secret=J5QXG4T5J5QXG4T5J5QXG4T5";
        let account = parse_key_uri(uri).expect("Failed to parse key URI");
        println!("{:?}", account);
    }
}
