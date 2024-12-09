pub mod TakeInput {
    fn takeInput() -> Result<i64, ()> {
        use std::io;
        let mut buffer = String::new();
        let stdin = io::stdin();
        let handle = stdin.lock();
        handle.read_line(&mut buffer)?;
        let input: &str = &buffer[..];
        let r_number = str::parse::<i64>();
        let number = match r_number {
            Ok(v) => v,
            Err(_) => {println!("Make sure you input a number"); self::takeInput()}
        };
    }
}
