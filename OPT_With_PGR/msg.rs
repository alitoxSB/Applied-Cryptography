

pub mod msg {
    pub fn menu (){
        println!("=====================================");
        println!("|| WELCOME TO ONE TIME PAD WITH PGR ||");
        println!("=====================================");
        println!("1: Watch text and key");
        println!("2: Encrypt text");
        println!("3: Decrypt text");
        println!("4: Exit");
    }
    pub fn input () -> String {
        use std::io;
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => println!("your input is: {}", i),
            Err(..) => println!("this was not an option: {}", trimmed),
        };
        input_text.trim().parse().unwrap()
    }
}