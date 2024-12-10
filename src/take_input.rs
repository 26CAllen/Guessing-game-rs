pub mod take_input {
    pub fn take_input() -> i64 {
        use text_io;
        let input: String = text_io::read!("{}\n");
        let r_number = input.parse::<i64>();
        let number = match r_number {
            Ok(v) => v,
            Err(_) => {println!("Make sure you input a number"); self::take_input(); 0}
        };
        return number
    }
}