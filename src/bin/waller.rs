use waller::terminal::Message;

extern crate waller;

pub fn main() {
    let result = waller::main();
    if let Err(e) = result {
        Message::fail(e.to_string().as_str());
    }
}
