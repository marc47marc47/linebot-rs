use base64::{Engine, engine::general_purpose::STANDARD};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(channel_secret: &str, body: &[u8], signature: &str) -> bool {
    let signature = match signature.strip_prefix("sha256=") {
        Some(sig) => sig,
        None => return false,
    };

    let decoded_signature = match STANDARD.decode(signature) {
        Ok(decoded) => decoded,
        Err(_) => return false,
    };

    let mut mac = match HmacSha256::new_from_slice(channel_secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };

    mac.update(body);

    mac.verify_slice(&decoded_signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_signature() {
        let channel_secret = "test_secret";
        let body = b"test_body";

        // Generate expected signature
        let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).unwrap();
        mac.update(body);
        let expected_signature = mac.finalize().into_bytes();
        let encoded_signature = STANDARD.encode(expected_signature);
        let signature_header = format!("sha256={}", encoded_signature);

        assert!(verify_signature(channel_secret, body, &signature_header));
        assert!(!verify_signature(channel_secret, body, "invalid_signature"));
    }
}
