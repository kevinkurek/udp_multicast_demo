use std::net::UdpSocket;
use std::{thread, time::Duration};

const MULTICAST_ADDR: &str = "239.192.1.1:6000"; 

pub fn run () {

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");
    socket.set_multicast_loop_v4(true).expect("Failed to set multicast loop");

    let mut seq_num = 1;

    loop {
        let message = format!("SEQ:{}|PRICE:{}", seq_num, 100.0+seq_num as f64);

        if seq_num % 5 != 0 {
            socket.send_to(message.as_bytes(), MULTICAST_ADDR).expect("Failed to send");
            println!("Sent: {}", message);
        } else {
            println!("Dropped packet SEQ:{}", seq_num); // "Dropped packet SEQ:10"

        }
        seq_num += 1;
        thread::sleep(Duration::from_millis(500));
    }

}

fn main() {
    run();
}