fn main() {
    let counter: i64 = 0;

    use rand;
    let answer = (rand::random() % 10000) + 1;
    gameLogic(input, answer, counter);
}

fn gameLogic(input: i64, answer: i64, mut counter: i64) {
    if input==answer {
        // win condition
        println!("Correct!");
        println("It took you")
        return;
    }

    counter += 1;

    match input > answer {
       True => println!("Too high!"),
       False => println!("Too low!")
    }
}