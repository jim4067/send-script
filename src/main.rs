extern crate autopilot;

use autopilot::*;

fn main() {
    let input = include_str!("joker.txt");
    let joker_script = String::from(input);

    for sentence in joker_script.lines() {
        autopilot::key::type_string(sentence, &[], 600.0, 0.0);
        autopilot::key::tap(&autopilot::key::Code(key::KeyCode::Return), &[], 10, 10);
    }
}
