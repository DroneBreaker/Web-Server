use std::net::TcpListener;
use std::io::Read;

//functionality for Struct - Server
// Self is used to point to the struct Server
pub struct Server {
    addr: String, 
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            // address is to address
            addr
        }
    }


    // run function
    pub fn run(self) {
        println!("Listening on: {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // to find a match - based on patterm match
            match listener.accept() {
                Ok((mut stream, _)) => {
                    //read the byte
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                        }

                        Err(e) => println!("Failed to read from conection: {}", e)
                    }
                }

                Err(e) => println!("Failed to establiesh a connection: {}", e)
            }

            // let result = listener.accept();

            // //  check to see if result is an error
            // //  and use the unwrap function
            // if result.is_err() {
            //     continue;
            // } 

            // let stream = result.unwrap();
        }

        //custom tuple
        // let tup = (5, "a", listener);
    }
}