use std::net::TcpListener;

//functionality for Struct - Server
// Self is used to point to the struct Server
pub struct Server {
    addr: String, 
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }


    // run function
    pub fn run(self) {
        println!("Listening on: {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {}
    }
}