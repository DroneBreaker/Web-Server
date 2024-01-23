fn main() {
    //Craete an enum instance
    let get = Method::GET;
    let post = Method::POST;
    let put = Method::PUT;
    let delete = Method::DELETE;
    let head = Method::HEAD;
    let connect = Method::CONNECT;
    let options = Method::OPTIONS;
    let trace = Method::TRACE;
    let patch = Method::PATCH;

    //Create a variable
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String, 
}

//functionality for Struct - Server
// Self is used to point to the struct Server
impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }

    fn run(self) {
        println!("Listening on: {}", self.addr)
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method
}

enum Method {
    GET,
    POST, 
    PUT, 
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl Request {
    
}
