const A_CODE: u8 = b'a';
const Z_CODE: u8 = b'z';
const MODULO: u8 = Z_CODE - A_CODE + 1;
const MIN_KEY: u8 = 0;
const MAX_KEY: u8 = MODULO - 1;

#[derive(Debug)]
enum CaesarError {
    InvalidPlainText(String),
    KeyOverflow(u8),
}

fn main() {
    let plain_text = String::from("rust");
    let key = 22;

    match caesar_encrypt(plain_text, key) {
        Ok(cipher_text) => {
            println!("Encrypt successfully!");
            println!(
                "Your cipher text is: {}", 
                cipher_text);
        }
        Err(CaesarError::InvalidPlainText(plain_text)) => {
            println!("Encrypt failed!");
            println!("Plaintext only contains 'a' -> 'z'");
            println!("Your plaintext = {}", plain_text);
        }
        Err(CaesarError::KeyOverflow(key)) => {
            println!("Encrypt failed!");
            println!(
                "Make sure your key is in range [{}..{}]", 
                MIN_KEY, 
                MAX_KEY);
            println!("Your key = {}", key);
        }
    }
}

fn caesar_encrypt(plain_text: String, key: u8) 
    -> Result<String, CaesarError> {
    // Validate the key
    if !validate_key(key) {
        return Err(CaesarError::KeyOverflow(key));
    }

    let mut cipher_text = String::new();

    for p in plain_text.chars() {
        // Validate the plain text character
        if !validate(p) {
            return Err(
                CaesarError::InvalidPlainText(plain_text)
            );
        }

        // Encrypt each character
        let p_code = p as u8;
        let c_code = (p_code - A_CODE + key) 
                        % MODULO + A_CODE;
        let c = c_code as char;

        cipher_text.push(c);
    }

    Ok(cipher_text)
}

// Validate if the character is within 'a' to 'z'
fn validate(c: char) -> bool {
    let c_code = c as u8;
    c_code >= A_CODE && c_code <= Z_CODE
}

// Validate the key range
fn validate_key(key: u8) -> bool {
    key >= MIN_KEY && key <= MAX_KEY
}