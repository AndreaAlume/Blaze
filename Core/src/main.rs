mod network;

fn main() {
    let server = network::server::start_server();

    if server.is_open {
        println!("Connection succesfully estabilished on: {} at port: {}", server.host, server.port);
    } else {
        println!("Connection refused. Check if there are some bugs in socket.rs")
    }

}
