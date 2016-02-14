mod keyboard;

use std::time::Duration;

fn main() {
	let keyboard = keyboard::create_keyboard_poller().unwrap();
	let key = keyboard.poll(Duration::from_millis(1000)).unwrap();
	println!("{}", key);
}