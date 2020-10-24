//! Pong server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "message" from client, replies with same "message"

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received: {}", msg.as_str().unwrap());
        responder.send(msg.as_str().unwrap(), 0).unwrap();
    }
}
