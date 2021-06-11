pub mod mik_api{
    extern crate openssl;
    extern crate serde;
    extern crate chrono;
    extern crate tiny_http;

    use std::time::{Duration, SystemTime, Instant};
    // use chrono::DateTime;
    use std::collections::HashMap;
    use core::str::from_utf8;
    use std::io::{Read, Write};
    use std::fs::File;
    use std::{net};
    use std::net::IpAddr;
    use std::net::TcpStream;
    use openssl::ssl;
    use openssl::ssl::{SslMethod, SslConnector, SslStream};

    /// Mikrotik connector main struct
    #[derive(Debug)]
    pub struct Connector{
        stream: Option<TcpStream>,
        ssl_stream: Option<SslStream<TcpStream>>,
        secured: bool,
        address: std::net::SocketAddr,
        username: Option<String>, // saves cridencials to restores session ( in development )
        password: Option<String>,
    }

    /// Commans config deserealization parental struct
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Commands{
        // interval: [u8; 4],
        commands: Vec<Queries>
    }

    /// Commands config deserealization structure
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Queries{
        graph_targets: Option<Vec<String>>,
        query: Option<Vec<String>>,
        separator: Option<String>,
        attributes: Option<Vec<String>>,
        // multiple_objects: bool,
        // frequency: [u8; 4],
        command: String,
        name: String,
    }

    /// Struct that contains user credentials
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Identity{
        name: String,
        uri: String,
        use_ssl: bool,
        username: String,
        password: String,
        cert: Option<String>, // cerification location of there is one
    }

    impl Connector{

        /// Initialization of connection object
        pub fn new(addr: std::net::SocketAddr, use_ssl: bool, verbose: bool) -> Result<Connector, String>{
            let connection;
            let stream = net::TcpStream::connect(addr).unwrap();
            if !use_ssl{ // if ssl is disabled
                connection = Connector{
                        stream: Some(stream),
                        ssl_stream: None,
                        username: None,
                        password: None,
                        secured: false,
                        address: addr
                    };
            }else{
                let mut connector = ssl::SslConnector::builder(SslMethod::tls_client()).unwrap();
                connector.set_verify(ssl::SslVerifyMode::NONE); // to make ssl verification !!!!! ( temporary solution )
                let connector = connector.build();
                connection = Connector{
                        stream: None,
                        ssl_stream: Some(connector.connect(&addr.ip().to_string(), stream).unwrap()),
                        username: None,
                        password: None,
                        secured: true,
                        address: addr
                    };
            }
            if verbose{
                println!("Connected to {}", addr);
            }
            Ok(connection)
        }


        /// Creates connactions using `credentials.json` file and at the same time logins them
        pub fn initial(file: String, save_credentials: bool, verbose: bool) -> Result<Vec::<Connector>, String>{
            let mut connections = Vec::new();
            let data: Vec::<Identity> = type_reader(&file);

            for item in &data{
                let mut connection = Connector::new( item.uri.parse().unwrap(), item.use_ssl, verbose ).unwrap();
                match connection.login(&item.username, &item.password, save_credentials, verbose){
                    Ok(_) => (),
                    Err(err) => println!("Error on logining {} one. Error name: \"{}\"", item.name, err)
                }
                connections.push(connection);
            }

            return Ok(connections);
        }

        /// Reads responce from the network stream after [Teller] send the request
        /// [Teller]: tell
        fn reader(&mut self) -> String{ // net::TcpStream
            let mut res_bytes = Vec::<u8>::new();        
            let mut data = [0 as u8; 1000]; // using 50 byte buffer
            
            if self.secured {
                loop{
                    match self.ssl_stream.as_mut().unwrap().read(&mut data) {
                        Ok(size) => {
                            for value in 0..size { res_bytes.push(data[value]); };
                            
                            if data[ size - 7..size] == [ 5, 33, 100, 111, 110, 101, 0 ] { break; } // '!done ' sign means end of sentence
                        },
                        Err(_) => { panic!("An error occurred, terminating connection"); }
                        }

                }
            } else { 
                // self.stream.as_mut().unwrap().set_read_timeout(Some(std::time::Duration::new(0, 50)));
                loop{
                    match self.stream.as_mut().unwrap().read(&mut data) {
                    Ok(size) => {
                        for value in 0..size { res_bytes.push(data[value]); };

                        if data[ size - 7..size] == [ 5, 33, 100, 111, 110, 101, 0 ] { break; } // '!done ' sign means end of sentence
                    },
                    Err(_) => { panic!("An error occurred, terminating connection"); }
                    }
            }}
            // println!("{:?}", res_bytes);
            bytes_to_str(&res_bytes)
        }
        
        /// Responce formater
        // fn responce_decoder(&mut self, responce: &str, attributes: Option<&Vec<String>>, key_values: &Vec<String>) -> Option::<Vec<HashMap::<String, String>>> { // temporary solution
        fn responce_decoder(&mut self, responce: &str, query: &Queries) -> Result::<HashMap::<String, String>, String> { // temporary solution
            let mut res         = HashMap::<String, String>::new();
            let mut landfill    = responce.split("\n");
            let     fst_value   = landfill.nth(0).unwrap();

            if  &fst_value.len() >= &5usize && &fst_value[..5] == "!done"{
                return Err(String::from("End message recieved"));
            }if &fst_value.len() >= &5usize && &fst_value[..5] == "!trap" || &fst_value.len() >= &6usize && &fst_value[..6] == "!fatal" {
                // panic!("Here is an error during parsing because of invalid responce {:?}", landfill);
                return Err(format!("{} responce from router", fst_value));
            }

            // let mut hashpiece = HashMap::new();
            let mut res_key          = String::from(format!("{}{{routerboard_address=\"{}\"", query.name, self.address.to_string()));
            let mut res_values       = Vec::<String>::new();
            for piece in landfill{
                {
                    if &piece[..] == "!re"{
                        if res_values.len() == 0{
                            res.insert(
                                res_key.to_owned()+"}",
                                0.to_string()
                            );
                        }else{
                            for value in &res_values{
                                res.insert(
                                    res_key.to_owned()+"}",
                                    value.to_string() 
                                );
                            }
                        }
                        res_key     = String::from(format!("{}{{routerboard_address=\"{}\"", query.name, self.address.to_string()));
                        res_values  = Vec::<String>::new();
                        // res.push(hashpiece);
                        // hashpiece = HashMap::new();
                        continue;
                    }
                    else if piece.len() >= 3 && &piece[..3] == ".id"{ // not quite sure it is valuable
                        
                        // res.insert(String::from(&piece[1..3]), String::from(&piece[5..]));
                        // hashpiece.insert(String::from(&piece[1..3]), String::from(&piece[5..]));
                    }else if piece.contains("=") {
                        let mut key = piece.split("=");
                        let (key, value) = (key.nth(0).unwrap(), key.nth(0).unwrap());
                        match query.graph_targets.as_ref(){
                            Some(val) => {
                                    if val.contains(&key.to_string()){
                                        res_values.push(String::from(value));
                                    }
                                },
                            None => ()/* println!("Error message from router") */
                        }
                        match &query.attributes{
                            Some(val) => {
                                if val.contains(&key.to_string()){
                                    res_key.push_str(&format!(", {}_{}=\"{}\"", query.name, key.replace("-", "_"), value));
                                }
                            },
                            None => { res_key.push_str(&format!(", {}_{}=\"{}\"", query.name, key.replace("-", "_"), value)) }
                        }
                        // if query.attributes != None && ( query.attributes.as_ref().unwrap().len() == 0 && value != "" ) || query.attributes.as_ref().unwrap().contains(&key.to_string()){
                        //     res_key.push_str(&format!(", {}_{}=\"{}\"", query.name, key.replace("-", "_"), value));

                        //     // res.insert(
                        //     //     String::from(key), 
                        //     //     String::from(value));
                        // }
                    }
                }
            }
            if res_values.len() == 0{
                res.insert(
                    res_key.to_owned()+"}",
                    0.to_string()
                );
            }else{
                for value in &res_values{
                    res.insert(
                        res_key.to_owned()+"}",
                        value.to_string() 
                    );
                }
            }

            Ok(res)
        }

        /// Logins into the routerboatd
        pub fn login(&mut self, username: &str, pwd: &str, overwrite: bool, verbose: bool) -> Result<(), String>{
            if self.username == None || overwrite == true { self.username = Some(String::from(username)); }
            if self.password == None || overwrite == true { self.password = Some(String::from(pwd)); }
            self.tell(&["/login".to_string(), format!("=name={}", self.username.as_ref().unwrap()), format!("=password={}", self.password.as_ref().unwrap())].to_vec(), verbose, None).unwrap();
            Ok(())
        }
        
        /// Sends commands to routerboard after [Login] has been perforned
        /// 
        /// [Login]: login
        pub fn tell(&mut self, lines: &Vec::<String>, verbose: bool, attributes: Option<&Vec<String>>) -> Result<String, String>{//sender: &mut [net::TcpStream]
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            text.push(0);
            if self.secured { (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap(); } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = self.reader();
            if verbose == true{
                println!(">> {}", &output);
            }
            Ok(output)
        }

        /// Sends commands from list to routerboard after [Login] has been perforned
        /// 
        /// [Login]: login
        pub fn tell_get(&mut self, lines: &Vec::<String>, verbose: bool, query: &Queries, hash_container: &mut Vec<HashMap<String, String>>) -> Result<(), String>{//sender: &mut [net::TcpStream]
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            text.push(0);
            if self.secured { (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap(); } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = self.reader();
            let hash_res = self.responce_decoder(&output[..], query);
            if verbose == true{
                match &hash_res{
                    Ok(val) => println!(">> {:#?}", val),
                    Err(msg) => println!("Error: {}", msg)
                }
            }

            match hash_res{
                Ok(value) => { hash_container.push(value); return Ok(()); },
                Err(msg) => Err(msg)
            }
        }
        
        /// Executes commands from list runs web server with metrics to be reserved by prometheus
        // pub async fn queries_teller(&mut self, queries: Commands, verbosibility: bool, uri: String, port: u32, ) -> Vec::<HashMap<String, String>> {
        pub async fn queries_teller(connections: &mut Vec::<Connector>, queries_file: String, verbosibility: bool, uri: String, port: u32, ) -> Vec::<HashMap<String, String>> {
            let mut metrics = Vec::<HashMap<String, String>>::new();    
            let server = tiny_http::Server::http(format!("{}:{}", uri, port)).unwrap();
            let mut queries: Commands = type_reader(&queries_file);

            loop{

                let request = match server.recv() {
                    Ok(rq) => { /* println!("{:?}", rq); */ rq },
                    Err(e) => { println!("error: {}", e); break }
                };

                match request.url() {
                    "/metrics" => {
                        println!("{:?}:\twent to metrics page", request);

                        // adding commands output
                        for connection in connections.iter_mut(){
                            for command in &queries.commands{
                                // println!("{}", command.command);
            
                                match connection.tell_get( &vec![ command.command.to_string() ], verbosibility, command, &mut metrics ){
                                    Ok(_) => (),
                                    Err(err) => println!("{:?}", err)
                                }
                                
                            }
                        }
                        // println!("{:?}", metrics);
        
                        let mut res = "".to_owned();
        
                        for dicts in &metrics{
                            for (key, value) in dicts{
                                res += &format!("{} {}\n", key, value);
                            }
                        }
        
                        let response = tiny_http::Response::from_string(res);
                        let _ = request.respond(response);
                        metrics = Vec::<HashMap<String, String>>::new();
                    },
                    "/" => { 
                        println!("{:?}: console home is here", request);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/index.html").unwrap())){
                            Ok(_) => (),
                            Err(e) => println!("Error happened: {}", e)
                        }
                    },
                    "/imgs/gears.gif" => {
                        println!("{:?}:\tgear gif file is here", request);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/images/gears.gif").unwrap())){
                            Ok(_) => (),
                            Err(e) => println!("Error happened: {}", e)
                        }
                    },
                    "/config/update" => {
                        println!("{:?}:\tupdating config", request);
                        queries = type_reader(&queries_file);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/reload_config.html").unwrap())){
                            Ok(_) => (),
                            Err(e) => println!("Error happened: {}", e)
                        }
                    }
                    _   => {
                        println!("{:?}", request);
                    }
                }

                // std::thread::sleep(date_array_to_duration(queries.interval)); // not used because prometheus will do it itself
            }
            return metrics;
        }
    }

    impl Drop for Connector{
        fn drop(&mut self) {
            match self.secured{
                true => { self.ssl_stream.as_mut().unwrap().shutdown()
                            .expect(&format!("Error during closing session {}", self.address.to_string())); },
                false => { self.stream.as_mut().unwrap().shutdown(std::net::Shutdown::Both)
                            .expect(&format!("Error during closing session {}", self.address.to_string())); }
            }
            println!("Disconnected from {}", self.address.to_string());
        }
    }

    /// Converts custum 4 elements date array to duration
    fn date_array_to_duration(time: [u8; 4]) -> Duration {
        Duration::new( (((( (time[0] as u64) * 24 )+ (time[1] as u64) * 60 )+ (time[2] as u64) * 60 ) + (time[3] as u64)) as u64, 0)
    }

    /// Reads data file and returns result
    pub fn type_reader<T>(file_name: &str) -> T where T: for<'de> serde::Deserialize<'de> {
        let file = std::fs::File::open(file_name).unwrap();
        let file_: T = serde_json::from_reader(file).unwrap();
        return file_;
    }

    /// Decodes responce to string
    fn bytes_to_str(bytes: &[u8]) -> String {

        let mut l = 0; // every word length
        let mut res = String::new();
        let mut iterator = 0; // iterrator for equal sign // temporary ( may be )

        for i in 0..bytes.len(){
            if i == 0 || l == 0 { if bytes[i] == 0 && i != bytes.len() - 1 { res += "\n"; continue; } else { l = bytes[i]; } }
            else { 
                l -= 1;
                match from_utf8(&[bytes[i]]){
                    Ok(val) => {
                        if val == "=" {
                            if iterator % 2== 0 { res+="\n" }
                            else { res+="="; } 
                            iterator += 1;
                        }
                        else{ res += val }
                    },
                    Err(e) => eprint!("Error during responce decoding: {}", e) 
                }
            }
        }

        res
    }
    /// Converts dec base to hex and adds length in the beginning as mikrotik api want
    fn dec_to_hec(mut value: usize) -> Vec::<u8>{
        let mut res = Vec::new();
        let too_high = value >= 268435456;
        while value <= 16 {
            res.push((((value / 16) % 16) * value % 16) as u8);
            value /= 256;
        }
        if too_high {
            res.reverse();
            res.push(240 as u8);
            res.reverse();
        }
        res
    }
    /// Converts dec array to hex array
    fn hexer(value: &[u8], add_last: bool) -> Vec::<u8>{
        let len = value.len();
        let mut res = Vec::<u8>::new();
        // println!("{}", len);
        
        if len < 128{
            res.push(len as u8);
        }else {
            res = dec_to_hec(len);
        }
        
        for val in value{
            res.push(*val);
        }
        
        if add_last{
            res.push(0);
        }
        res
    } 

}