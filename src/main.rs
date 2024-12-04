fn main() {
    println!("Hello, world!");
}

fn gameLogic(str_input: &str) {
    let input_res = str_input.parse::<i32>();
    let input = match input_res {
        Ok(v) => v,
        Err(_) => {, 1}
    }
}