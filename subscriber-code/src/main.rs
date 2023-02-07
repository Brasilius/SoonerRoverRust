use std::io;
use std::str;
use zmq::Context;
use zmq::SUB;

fn main() {
    println!("Subscriber code test");
    let context = zmq::Context::new;
    let socket = context().socket(zmq::SUB).unwrap();
    socket.bind("tcp://*:8888").unwrap();
    let mut input = String::new();

    loop {
        println!("Continue running (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("issue reading input");
        if input == "true".to_string() {
            println!("okay!")
        } else {
            println!("NOOOO!");
            break;
        }
    }
}
