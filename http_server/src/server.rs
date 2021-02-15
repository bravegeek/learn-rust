use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    addr: String,
}


impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
               Ok((mut stream, _)) => {
                   let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            // Request::try_from(&buffer as &[u8]);
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse Request: {}", e),
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e)

                // // also valid:
                // Ok((tup)) => {let (stream, addr) = tup;}
                // // default case:
                // _ =>
            }
        }

    }
}

// this can take a pointer to an array
// aka, an array slice (like a string slice)
fn arr1(a: &[u8]){}

fn do_something_w_array(){
    let a1 = [1,2,3,4,5];
    arr1(&a1);
}

        // // declaring a tuple:
        //let tup = (5, "a", listener);

        // // labeled loops
        // 'outer: loop {
        //     loop {
        //         break: 'outer;
        //     }
        // }

        // // an infinite loop
        // while true {
        // }
