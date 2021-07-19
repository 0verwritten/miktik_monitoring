extern crate async_std;

use miktik_api::miktik_api::Connector;
use std::{ io, env, sync::{ Arc, Mutex } };

/// brings interactiveness to the application
fn interactive(mut connections: std::vec::Vec<Connector>){
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

    let a: Vec<Mutex<Connector>> = connections.into_iter().map(|x| Mutex::from(x)).collect();
}

#[async_std::main]
async fn main(){

    if env::args().filter( |x| {x == "-h" || x == "--help"} ).count() > 0 {
        println!("Help:\n-i\t\tget interactive console\n-h\t\tget some help\n-q <file name>\texecute commands from file ( not developed yet )\n-c <file name>\tcridentials file ( not even close to be done )\n");
        println!("Enviroment variables example:\nweb_server_address=0.0.0.0\nweb_server_port=7878");
        println!("\nHelp for interactive console:\n? - to get help again\n. - to stop\n<command> - to write command\nall queries that you want to put into the function must be on the next line\nTo send command press Enter ( new line ) again");
    
        return;
    }

    let connections = Connector::initial( String::from("./config/credentials.json"), false ).unwrap();
    // let mut connections = Connector::initial_mutex( String::from("./config/credentials.json"), true, false ).unwrap();

    let uri: String = match env::vars_os().find(|x| { x.0  == "web_server_address"}){
        Some(val) => val.1.into_string().unwrap(),
        None => String::from("0.0.0.0")
    };
    let port: u32 = match env::vars_os().find(|x| { x.0 == "web_server_port"}){
        Some(val) => val.1.into_string().unwrap().parse().unwrap(),
        None => 7878
    };

    if env::args().filter( |x| {x == "-i"} ).count() > 0 {
        interactive(connections);
    }else if env::args().filter( |x| {x == "-v"} ).count() > 0 {
        let connections: Vec<Mutex<Connector>> = connections.into_iter().map(|x| Mutex::from(x)).collect();
        Connector::queries_teller(Arc::new(connections), "./config/commands.json".to_string(), true, uri, port).await;
    }
    else{
        let connections: Vec<Mutex<Connector>> = connections.into_iter().map(|x| Mutex::from(x)).collect();
        Connector::queries_teller(Arc::new(connections), "./config/commands.json".to_string(), false, uri, port).await;
    }
}