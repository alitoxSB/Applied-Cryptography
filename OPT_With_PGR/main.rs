use crate::encrypt::encrypt::{encryption, string_count};
use crate::decrypt::decrypt::decrypt;
use crate::msg::msg::{menu, input};
use crate::pgr::pgr::pgr_function;

mod msg;
mod encrypt;
mod decrypt;
mod pgr;


fn main() {
    use std::io::{stdin,stdout,Write};

    let mut text = "Ned Flanders";
    let mut key = "Homer";
    let mut option = 0;

    while  (option < 4) {
        menu();
        option = input().parse().unwrap();
        if option == 1{
            println!("The text is: {}",text);
            println!("The key is: {}",key);
        }
        if option == 2{
            let text_bytes = text.as_bytes();
            let cypher_text = encryption(text.parse().unwrap(), key.parse().unwrap(), text_bytes);
            println!("Encrypting text...");
            println!("The Cyphertext is: {:?}",cypher_text);
        }
        if option == 3{
            let text_bytes = text.as_bytes();
            let cypher_text = encryption(text.parse().unwrap(), key.parse().unwrap(), text_bytes);
            let diference = string_count(text.parse().unwrap()) - string_count(key.parse().unwrap());
            let key_string = pgr_function(key.parse().unwrap(), text.parse().unwrap(), diference);
            let decrypt_text = decrypt(&*cypher_text, &*key_string);
            let text = String::from_utf8_lossy(&*decrypt_text);
            println!("The Decrypt Text is: {}", text);
        }
    }


}


