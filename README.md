# UDP Multicast Demo

This project demonstrates a UDP multicast server (SIP Feed) and client (Hedge Fund, market maker, investment bank, etc..), along with a TCP recovery server. The UDP multicast server sends multicast packets, the client receives them, and requests missing packets from the TCP recovery server if needed.

## Example Output

When running the project, you should see output similar to the following:

```
// simulate realtime data from udp_server (SIP Feed: NYSE, AMEX, NASDAQ)
// $ cargo run --bin udp_multicast_server
Sent: SEQ:6|PRICE:106
Sent: SEQ:7|PRICE:107
Sent: SEQ:8|PRICE:108
Sent: SEQ:9|PRICE:109
Dropped packet SEQ:10 // NOTE: server doesn't actually drop packets, it's simulating the udp_client not receiving this packet.

// data received by udp_client (Hedge Fund, Market Maker, Investment Bank, etc..)
// $ cargo run --bin udp_multicast_client
Received: SEQ:7|PRICE:107
Received: SEQ:8|PRICE:108
Received: SEQ:9|PRICE:109
Received: SEQ:11|PRICE:111
⚠️ Missing packet(s)! Requesting SEQ:10 from recovery server...
Sending recovery request: GET SEQ:10
✅ Recovered: Recovered data for SEQ:10 // comes after tcp_recovery sends data back

// tcp_recovery_server acknowledging a request for dropped packets
// $ cargo run --bin tcp_recovery_server
Received request: GET SEQ:1
Received request: GET SEQ:10
```


## **Background**

For those newer to SIP feeds (Securities Information Processors), there are three core SIP feeds: Tape A, Tape B, and Tape C, which distribute consolidated market data for U.S. equities.

**Tape A**: Covers securities listed on the **New York Stock Exchange (NYSE)**.
**Tape B**: Covers securities listed on ***regional exchanges and the NYSE American (formerly AMEX)
Tape C***: Covers securities listed on the **Nasdaq Stock Market**.

Each of these feeds aggregates trade and quote data from multiple exchanges, ensuring that all market participants have access to the best bid and offer prices, as well as executed trades, in a consolidated manner. The SIPs act as the official source for National Best Bid and Offer (NBBO) data under U.S. regulations.

SIP feeds are distributed using UDP multicast, which allows for efficient one-to-many data transmission across trading firms, market makers, and other participants. However, UDP does not provide guaranteed delivery, meaning packet loss can occur due to network congestion, routing issues, or hardware failures. To mitigate this, a **TCP recovery server** allows clients to request retransmission of missing packets, ensuring completeness and accuracy of market data.

This project simulates a SIP-like environment with a **UDP multicast server**, a **UDP multicast client**, and a **TCP-based recovery server**. The multicast server continuously transmits simulated market data, the client listens for and processes incoming packets, and if any packets are lost, the client connects to the TCP recovery server to request missing data.

This setup closely mirrors real-world SIP feed distribution, where financial institutions implement robust networking and failover strategies to minimize latency and ensure high reliability in market data consumption.

## Project Structure

- `src/main.rs`: The main entry point of the application, which runs all components concurrently.
- `src/udp_multicast_server.rs`: Module for the UDP multicast server.
- `src/udp_multicast_client.rs`: Module for the UDP multicast client.
- `src/tcp_recovery_server.rs`: Module for the TCP recovery server.

## Requirements

- Rust (latest stable version)
- Tokio (asynchronous runtime for Rust)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/udp_multicast_demo.git
   cd udp_multicast_demo
   ```
2. Build the project:

   ```bash
   cargo build
   ```

## Running the Project

To run the project, use the following command(s):

```bash
// approach 1: run all servers and client together to make the project "just work." 
// This is harder to visual (blocking threads were created to simulate each of these running on different hardware.)
cargo run

// approach 2: individually run the tcp_recovery_server, udp_multicast_server, and udp_multicast_client in 3 separate terminals. 
// This provides a better visual for when the udp_server is sending, udp_client is receiving, and tcp_recovery is being requested.
cargo run --bin tcp_recovery_server
cargo run --bin udp_multicast_server
cargo run --bin udp_multicast_client
```

This will start the TCP recovery server, UDP multicast server, and UDP multicast client concurrently.

## Modules

### UDP Multicast Server

The [udp_multicast_server](vscode-file://vscode-app/Applications/Visual%20Studio%20Code.app/Contents/Resources/app/out/vs/code/electron-sandbox/workbench/workbench.html) module simulates a data source sending multicast packets. It sends packets with a sequence number and price, and drops every 5th packet to simulate packet loss.

### UDP Multicast Client

The [udp_multicast_client](vscode-file://vscode-app/Applications/Visual%20Studio%20Code.app/Contents/Resources/app/out/vs/code/electron-sandbox/workbench/workbench.html) module receives multicast packets and requests missing packets from the TCP recovery server if needed.

### TCP Recovery Server

The [tcp_recovery_server](vscode-file://vscode-app/Applications/Visual%20Studio%20Code.app/Contents/Resources/app/out/vs/code/electron-sandbox/workbench/workbench.html) module simulates a database with recovery data. It listens for TCP connections and responds to requests for missing packets.
