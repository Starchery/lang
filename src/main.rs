mod token;

fn main() {
    println!("Hello, world!");
    println!("{:?}", token::Token::Int("34".parse::<i64>().unwrap()));
}

