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

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*
GET /path?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/


fn string_learning() {

    // a proper String struct
    let string = String::from("127.0.0.1:8080");

    // '&str' is a string slice, an immutable view into a string
    // By nature it's a reference/pointer
    let string_slice = &string[10..14];
    let string_borrow: &str = &string;

    // literal strings are also &str, string slices
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
}