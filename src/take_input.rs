pub mod take_input {
    pub fn take_input() -> i64 {
        use std::io::BufRead;
        use std::io;
        let mut buffer = String::new();
        let stdin = io::stdin();
        let result = stdin.read_line(&mut buffer);
        match result {
            Ok(_) => {}
            Err(_) => {println!("Make sure you input a number"); self::take_input();}
        };
        let input: &str = &buffer[..];
        let r_number = str::parse::<i64>(input);
        let number = match r_number {
            Ok(v) => v,
            Err(_) => {println!("Make sure you input a number"); self::take_input(); 0}
        };
        return number
    }
}