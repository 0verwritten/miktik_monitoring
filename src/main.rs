use core::str::from_utf8;
use mik_api::mik_api::Connector;
use std::net::SocketAddr;
use std::io;
/*
TO DO:

_ make some comments + function documentation
_ remove length mark or use it correctly in router's reply
_ async requests or parallel
_ move enerything into a library
+ ssl encryption
_ rename functions to make them more meaningful
_ ssl ca verification 
_ ssl certificate acceptance
_ make error type, not just a string
_ utf8 error handling

_ do all mikrotik queriy types

*/

fn main(){
    let addrs = [
            // net::SocketAddr::from(([10, 54, 71, 3], 8728)),
            // net::SocketAddr::new("10.13.40.8".parse().unwrap(), 8728),
            SocketAddr::new("10.13.40.8".parse().unwrap(), 8728),
        ];
    let login = "user1";
    let pass = "123";

    let mut connections: Vec::<Connector> = Connector::new(&addrs, false, true).unwrap();

    for i in 0..connections.len(){
        connections[i].login(login, pass, false, true).expect("Login error");
    }

    let mut lines = Vec::new();
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if  line == "."{
            break;
        }else if line == ""{
            lines.push(line.to_string());
            for i in 0..connections.len(){
                connections[i].tell(&lines, true).expect("Tell error");
            }

            lines.clear();
        }else{
            lines.push(line.to_string());
        }
    }

    println!("Session ended");
}