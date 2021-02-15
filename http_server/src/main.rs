use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;



fn main() {

    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    
    // the '::' (double colon) calls an associated function
    // an associated function is similar to a static function in java
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}


