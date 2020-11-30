pub mod decrypt {
    use crate::pgr::pgr::pgr_function;

    pub fn decrypt (cypher_text : &[u8], key_string : &[u8]) -> Vec<u8> {
        let mut xor: Vec<u8> = Vec::new();
        for n in 0..cypher_text.len() {
            xor.push(cypher_text[n] ^ key_string[n]);
        }
        xor.to_vec()
    }
}