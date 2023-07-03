# rsip

       _____   _____ _____ _____  
     |  __ \ / ____|_   _|  __ \ 
     | |__) | (___   | | | |__) |
     |  _  / \___ \  | | |  ___/ 
     | | \ \ ____) |_| |_| |     
     |_|  \_\_____/|_____|_|   


A lightweight Network helper written in Rust. It helps verifying your network setup by mirroring the connection infos of an accessing device. I made this tool to troubleshoot my network setups during AZ-700 exam preparation. I am totally aware that there are already tools with similar functionality. 😉

## Usage
The tool enters an infinite loop and listens on port 8080. It prints out the connection infos of the accessing device. The tool is written in Rust and is therefore very lightweight. It is a single binary and does not require any dependencies. I plan to place it onto provisioned VMs to verify the network setup via terraform.

This binary is part of my upcoming blog posts about the AZ-700 exam. Stay tuned!

## Installation
I assume that you have already installed Rust on your machine. If not, please follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

1. Clone the repository and build the binary.

```bash
$ git clone
$ cd rsip
$ cargo build --release
```

2. Place the binary in your $PATH

```bash
$ sudo cp target/release/rsip /usr/local/bin
```

3.  make it executable.

```bash
$ cd /usr/local/bin
$ chmod +x rsip
```

4. Run the tool

```bash
$ rsip
```

## Example output (console)

```bash
$ rsip

[2023-07-03T13:33:53Z INFO  rsip] Connection accepted
[2023-07-03T13:33:53Z INFO  rsip] IP address: 127.0.0.1
[2023-07-03T13:33:53Z INFO  rsip] Port number: 59046
[2023-07-03T13:33:53Z WARN  rsip] Connection closed
```
## Example output (browser)

```bash
$ curl localhost:8080

<html><body><h1>RSIP</h1><table><tr><th>Source IP</th><th>Source Port</th>
                            </tr><tr><td>127.0.0.1</td><td>44590</td></tr><tr>
                            <th>Destination IP</th><th>Destination Port</th>
                            </tr><tr><td>127.0.0.1</td><td>8080</td></tr>
curl: (56) Recv failure: Connection reset by peer
                            </table></body></html>%   
```

which is rendered in your browser as:

![RSIP](/rsip_browser.png)