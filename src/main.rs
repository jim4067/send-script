use enigo::{Enigo, Key, KeyboardControllable};

fn main() {
    let mut enigo = Enigo::new();
    let input = include_str!("joker.txt");

    for sentence in input.to_string().lines() {
        enigo.key_sequence(sentence);
        enigo.key_click(Key::Return);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // for _ in 0..2000 {
    //     enigo.key_sequence("🤣 🤣 🤣 🤣 🤣 🤣");
    //     enigo.key_click(Key::Return);
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    // }
}
