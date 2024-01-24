use server::Server;
use http::Request;
use http::Method;

mod server;
mod http;

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
    let server = Server::new("127.0.0.1:8081".to_string());
    server.run();
}

