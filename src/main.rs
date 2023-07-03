/*
    RSIP is a simple IP adress verification tool that I made
    during my AZ-700 exam preparation. It is a simple tool that
    can be used to verify your network configurations by printing
    the accessor's IP address and port number via browser access.

 */

use std::io::Write;
use env_logger;
use log::{info, warn};

fn main() {

    // Initialize the logger
    env_logger::init();

    // Create a server socket and bind it to port 80
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind to port 8080");

    // Get the IP address and port number of the server
    let dest_ip = listener.local_addr()
        .expect("Failed to get IP address")
        .ip();
    let dest_port = listener.local_addr()
        .expect("Failed to get port number")
        .port();

    #[warn(while_true)]
    loop {
        // Accept incoming connections
        let (mut stream, _) = listener.accept()
            .expect("Failed to accept connection");

        // Get the IP address and port number of the accessor
        let source_ip = stream.peer_addr()
            .expect("Failed to get IP address")
            .ip();
        let source_port = stream.peer_addr()
            .expect("Failed to get port number")
            .port();

        // Print the IP address and port number
        info!("Connection accepted");
        info!("IP address: {}", source_ip);
        info!("Port number: {}", source_port);

        // Send a response to the accessor
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes())
            .expect("Failed to send response");

        // Send a HTML response to the accessor with the relevant information
        let html = format!("<html><body><h1>RSIP</h1><table><tr><th>Source IP</th><th>Source Port</th>
                            </tr><tr><td>{}</td><td>{}</td></tr><tr>
                            <th>Destination IP</th><th>Destination Port</th>
                            </tr><tr><td>{}</td><td>{}</td></tr>
                            </table></body></html>", source_ip, source_port, dest_ip, dest_port);

        stream.write(html.as_bytes())
            .expect("Failed to send response");

        // Close the connection
        drop(stream);

        warn!("Connection closed");

    }
}