use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

async fn handle_client(mut stream: TcpStream, data_store: Arc<RwLock<HashMap<u32, String>>>) {
    let mut buffer = [0; 1024];

    // match the stream.read() to get the data from the client; reads the data into the buffer
    match stream.read(&mut buffer).await {
        Ok(size) => {
            if size > 0 {
                let request = String::from_utf8_lossy(&buffer[..size]);
                println!("Received request: {}", request); // Received request: GET SEQ:10

                if request.starts_with("GET SEQ:") {
                    let seq_num: u32 = request[8..].trim().parse().unwrap_or(0);
                    let response = {

                        // Read Lock: RwLock::read()
                        // Must be Arc<RwLock<>> to be shared between threads
                        // A read lock allows multiple threads to read from the shared data concurrently. 
                        // When a read lock is acquired, other threads can also acquire read locks, 
                        // but no thread can acquire a write lock until all read locks are released.
                        let store = data_store.read().unwrap(); // Use read lock

                        // Write Lock: RwLock::write()
                        // A write lock allows a single thread to write to the shared data. When a write lock is acquired, 
                        // no other thread can acquire a read or write lock until the write lock is released.

                        // get the recovery data from the data store
                        store.get(&seq_num).cloned().unwrap_or_else(|| "Not found".to_string())
                    };

                    // send the response back to the client
                    stream.write_all(response.as_bytes()).await.expect("Failed to send response");
                }
            }
        },
        Err(e) => {
            println!("Failed to read from stream: {}", e);
        }
    }
}

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:7000").await.expect("Could not bind");
    let data_store: Arc<RwLock<HashMap<u32, String>>> = Arc::new(RwLock::new(HashMap::new()));

    // simulate a tcp database that has the data needed for recovery if udp packets are lost
    {
        // has to block the store so that the lock is released before the next write
        let mut store = data_store.write().unwrap(); // Use write lock
        store.insert(1, "Recovered data for SEQ:1".to_string());
        store.insert(10, "Recovered data for SEQ:10".to_string());
        store.insert(15, "Recovered data for SEQ:15".to_string());
        store.insert(20, "Recovered data for SEQ:20".to_string());
        store.insert(25, "Recovered data for SEQ:25".to_string());
    }

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {

                // Clone the data store to be moved into the async task
                let data_store = Arc::clone(&data_store);

                // Spawn a new async task to handle the client request
                tokio::spawn(async move {
                    handle_client(stream, data_store).await;
                });
            },
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}