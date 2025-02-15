use tokio::task;

mod udp_multicast_server;
mod udp_multicast_client;
mod tcp_recovery_server;

#[tokio::main]
async fn main() {
    // Spawn the TCP recovery server to simulate a database with recovery data
    let tcp_recovery_server_handle = tokio::spawn(async {
        tcp_recovery_server::run().await;
    });

    // Spawn the UDP multicast server to simulate a data source sending multicast packets
    let udp_multicast_server_handle = task::spawn_blocking(|| {
        udp_multicast_server::run();
    });

    // Spawn the UDP multicast client to simulate a client receiving multicast packets
    // and requesting recovery data from the TCP recovery server
    let udp_multicast_client_handle = task::spawn_blocking(|| {
        udp_multicast_client::run();
    });

    // Wait for all tasks to complete so the program does not exit prematurely
    let _ = tokio::join!(
        tcp_recovery_server_handle,
        udp_multicast_server_handle,
        udp_multicast_client_handle
    );
}
