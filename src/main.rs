extern crate async_std;

use mik_api::mik_api::queries_reader;
use std::{ io, env, future, task };
use mik_api::mik_api::Connector;
use std::net::SocketAddr;
use core::str::from_utf8;

/// brings interactiveness to the application
fn interactive(mut connections: std::vec::Vec<mik_api::mik_api::Connector>){
    let mut lines = Vec::new();
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if  line == "."{
            break;
        }else if line == "?"{
            println!("Help:\n? - to get help again\n. - to stop\n<command> - to write command\nall queries that you want to put into the function must be on the next line\nTo send command press Enter ( new line ) again");
        }else if line == ""{
            lines.push(line.to_string());
            for i in 0..connections.len(){
                connections[i].tell(&lines, true, None).expect("Tell error");
            }

            lines.clear();
        }else{
            lines.push(line.to_string());
        }
    }
    println!("Session ended");
}

#[async_std::main]
async fn main(){

    if env::args().filter( |x| {x == "-h" || x == "--help"} ).count() > 0 {
        println!("Help:\n-i\t\tget interactive console\n-h\t\tget some help\n-q <file name>\texecute commands from file ( not developed yet )\n-c <file name>\tcridentials file ( not even close to be done )");
        println!("\nHelp for interactive console:\n? - to get help again\n. - to stop\n<command> - to write command\nall queries that you want to put into the function must be on the next line\nTo send command press Enter ( new line ) again");
    
        return;
    }

    let addrs = [
            SocketAddr::new("<in address>".parse().unwrap(), 8729),
        ];
    let login = "user1";
    let pass = "123";

    let mut connections: Vec::<Connector> = Connector::new(&addrs, true, true).unwrap();
    let connections_len = connections.len();

    for i in 0..connections.len(){
        connections[i].login(login, pass, false, true).expect("Login error");
    }

    if env::args().filter( |x| {x == "-i"} ).count() > 0 {
        interactive(connections);
    }else{
        Connector::queries_teller(&mut connections, "commands.json".to_string(), false, "localhost".to_string(), 7878).await;
    }
}