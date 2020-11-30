use std::str::from_utf8;
use std::str;
use std::vec::Vec;
pub mod encrypt {
    use crate::pgr::pgr::pgr_function;
    use crate::decrypt::decrypt::decrypt;

    pub fn encryption(text: String, key: String, text_bytes: &[u8]) -> Vec<u8>{

        let diference = text.chars().count() - key.chars().count();
        let len_key = key.chars().count();
        let key_chars: Vec<char> = key.chars().collect();

        let key_string = pgr_function(key, text, diference);
        let mut cypher_text = xor_encrypt(text_bytes, &*key_string);

        decrypt(&*cypher_text, &*key_string);
        let decrypt_text = decrypt(&*cypher_text, &*key_string);
        let text = String::from_utf8_lossy(&*decrypt_text);
        cypher_text

    }

    fn xor_encrypt(text_bytes: &[u8], final_key_as_byte: &[u8]) -> Vec<u8> {
        let mut xor: Vec<u8> = Vec::new();
        for n in 0..text_bytes.len() {
            xor.push(text_bytes[n] ^ final_key_as_byte[n]);
        }
        xor.to_vec()
    }
    pub fn string_count(text:String) -> usize{
        let diference : usize = text.chars().count() as usize;
        diference
    }

}