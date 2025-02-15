use core::str;
use std::net::{UdpSocket, Ipv4Addr, TcpStream};
use std::io::{Write, Read};

// const MULTICAST_ADDR: &str = "239.192.1.1:6000";
const RECOVERY_SERVER: &str = "127.0.0.1:7000";

fn recover_missing_packet(seq_num: u32) {
    // Attempt to connect to the recovery server
    if let Ok(mut stream) = TcpStream::connect(RECOVERY_SERVER) {
        // Format the recovery request with the missing sequence number
        let request = format!("GET SEQ:{}", seq_num); // Example: GET SEQ:10
        
        // Send the recovery request to the server 
        // There is a TcpListener::bind() in tcp_recovery_server.rs picking up this write request
        println!("Sending recovery request: {}", request);
        stream.write_all(request.as_bytes()).expect("Failed to send recovery request");

        // Buffer to store the response from the server
        let mut buffer = [0; 1024];
        
        // Read the response from the server; this came from a stream.write_all() in tcp_recovery_server.rs
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    // Convert the response to a string and print it
                    let recovered_data = str::from_utf8(&buffer[..size]).unwrap();
                    println!("✅ Recovered: {}", recovered_data);
                } else {
                    // No data received for the requested sequence number
                    println!("❌ No data received for SEQ:{}", seq_num);
                }
            },
            Err(e) => {
                // Handle any errors that occur while reading the response
                println!("❌ Failed to recover SEQ:{} due to error: {}", seq_num, e);
            }
        }
    } else {
        // Handle the case where the connection to the recovery server fails
        println!("❌ Could not connect to recovery server.");
    }
}

pub fn run () {
    let socket = UdpSocket::bind("0.0.0.0:6000").expect("Could not bind socket");
    socket.join_multicast_v4(&Ipv4Addr::new(239, 192, 1, 1), 
    &Ipv4Addr::new(0,0,0, 0)).expect("Could not join multicast group");

    let mut buf = [0; 1024];
    let mut expected_seq = 1;

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size,_)) => {
                let data = str::from_utf8(&buf[..size]).expect("No Data Received.");  // SEQ:7|PRICE:107
                let parts: Vec<&str> = data.split('|').collect();                               // ["SEQ:7", "PRICE:107"]
                let seq_part = parts[0].split(':').nth(1).unwrap();                       // "SEQ:7" => ["SEQ", "7"] => "7"
                let seq_num: u32 = seq_part.parse().unwrap();                                   // "7" => 7

                println!("Received: {}", data);

                if seq_num != expected_seq // 11 != 10
                { 
                    println!("⚠️ Missing packet(s)! Requesting SEQ:{} from recovery server...", expected_seq); // "Missing 10, Requesting"
                    recover_missing_packet(expected_seq);
                }
                expected_seq = seq_num + 1;
            }
            Err(_) => {
                println!("Error receiving multicast packet.")
            }
        }
    }
    
}

fn main() {
    run();
}