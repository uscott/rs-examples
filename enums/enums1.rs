// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
	Quit = 0,
	Echo = 1,
	Move = 2,
	ChangeColor = 3,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
