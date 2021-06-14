pub mod mik_api{
    extern crate openssl;
    extern crate serde;
    extern crate chrono;
    extern crate tiny_http;
    extern crate termion;

    use std::{
        time::{Duration/*, SystemTime, Instant*/},
        collections::HashMap,
        io,
        io::{Read, Write},
        // fs::File,
        net,
        net::{/*IpAddr,*/ TcpStream}};
    // use chrono::DateTime;
    use core::str::from_utf8;
    use termion::{color, style};
    use openssl::{ssl, ssl::{SslMethod, /*SslConnector,*/ SslStream}};

    /// Responce Error type
    pub enum ConnectionError{
        ResponceError(String),
        IoError(io::Error)
    }

    /// Mikrotik connector main struct
    #[derive(Debug)]
    pub struct Connector{
        stream: Option<TcpStream>,
        ssl_stream: Option<SslStream<TcpStream>>,
        address: std::net::SocketAddr,
        username: Option<String>, // saves cridencials to restores session ( in development )
        password: Option<String>,
        cert: Option<String>, // cerification location of there is one
        ca_cert: Option<String>,
        secured: bool
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
        split_attributes: Option<Vec<String>>,
        split_character: Option<String>,
        graph_targets: Option<Vec<String>>,
        attributes: Option<Vec<String>>,
        query: Option<Vec<String>>,
        separator: Option<String>,
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
        ca_cert: Option<String>
    }

    impl Connector{

        /// Initialization of connection object
        pub fn new(addr: std::net::SocketAddr, use_ssl: bool, verbose: bool, cert_file: Option<&String>, ca_cert_file: Option<&String>) -> Result<Connector, io::Error>{
            let connection;
            let stream = match net::TcpStream::connect_timeout(&addr, Duration::new(2, 2)){
                Ok(con) => con,
                Err(err) => return Err(err)
            };
            stream.set_read_timeout(    Some(Duration::new(2,0))   ).unwrap();
            stream.set_write_timeout(   Some(Duration::new(2,0))   ).unwrap();

            if !use_ssl{ // if ssl is disabled
                connection = Connector{
                        stream: Some(stream),
                        ssl_stream: None,
                        username: None,
                        password: None,
                        secured: false,
                        address: addr,
                        ca_cert: None,
                        cert: None
                    };
            }else{
                let mut connector = ssl::SslConnector::builder(SslMethod::tls_client()).unwrap();
                if let Some(ca_file) = ca_cert_file {
                    connector.set_ca_file( std::path::Path::new(ca_file)).unwrap();
                    connector.set_verify(ssl::SslVerifyMode::PEER); 
                }else if let Some(cert) = cert_file{
                    connector.set_certificate_file( std::path::Path::new(cert), ssl::SslFiletype::PEM ).unwrap();
                    connector.set_verify(ssl::SslVerifyMode::PEER); 
                } else { 
                    eprintln!("{}Warning!{} No certificate verification used", color::Fg(color::Yellow), color::Fg(color::Reset));
                    connector.set_verify(ssl::SslVerifyMode::NONE);
                }
                // connector.set_verify()
                let connector = connector.build();
                connection = Connector{
                        stream: None,
                        ssl_stream: Some(connector.connect(&addr.ip().to_string(), stream).unwrap()),
                        username: None,
                        password: None,
                        secured: true,
                        address: addr,
                        ca_cert: match ca_cert_file { Some(val) => Some(String::from(val)), None => None },
                        cert: match cert_file { Some(val) => Some(String::from(val)), None => None }
                    };
            }
            println!("{}Connected{} to {}", color::Fg(color::LightGreen), color::Fg(color::Reset), addr);
            Ok(connection)
        }


        /// Creates connactions using `credentials.json` file and at the same time logins them
        pub fn initial(file: String, save_credentials: bool, verbose: bool) -> Result<Vec::<Connector>, String>{
            let mut connections = Vec::new();
            let data: Vec::<Identity> = type_reader(&file);

            for item in &data{
                let mut connection = match Connector::new( match item.uri.parse() { Ok(val) => val, Err(msg) => return Err(msg.to_string()) }, item.use_ssl, verbose, item.cert.as_ref(), item.ca_cert.as_ref() ){
                    Ok(val) => val,
                    Err(err) => { eprintln!("{}Error{} connecting to {}: {}. Skipping", color::Fg(color::LightRed), color::Fg(color::Reset), item.uri.parse::<net::SocketAddr>().unwrap(), err); continue; }
                };
                match connection.login(&item.username, &item.password, save_credentials, verbose){
                    Ok(_) => (),
                    Err(err) => eprintln!("{}Error{} on logining {} one. Error name: \"{}\"", color::Fg(color::LightRed), color::Fg(color::Reset), item.name, err)
                }
                connections.push(connection);
            }

            return Ok(connections);
        }

        fn reconnect(&mut self, verbose: bool) -> Result<(), ConnectionError>{
            let credentials = (self.username.as_ref().unwrap().to_string(), self.password.as_ref().unwrap().to_string());
            match Connector::new(self.address, self.secured, verbose, self.cert.as_ref(), self.ca_cert.as_ref()){
                Ok(val) => *self =  val,
                Err(err) => return Err(ConnectionError::IoError(err))
            }
            match self.login(&credentials.0, &credentials.1, true, verbose){
                Ok(_) => Ok(()),
                Err(err) => Err(ConnectionError::ResponceError(err))
            }
        }

        /// Reads responce from the network stream after [Teller] send the request
        /// [Teller]: tell
        fn reader(&mut self) -> Result<String, io::Error>{ // net::TcpStream
            let mut res_bytes = Vec::<u8>::new();        
            let mut data = [0 as u8; 1000]; // using 50 byte buffer
            
            if self.secured {
                loop{
                    match self.ssl_stream.as_mut().unwrap().read(&mut data) {
                        Ok(size) => {
                            for value in 0..size { res_bytes.push(data[value]); };
                            
                            if data[ size - 7..size] == [ 5, 33, 100, 111, 110, 101, 0 ] { break; } // '!done ' sign means end of sentence
                        },
                        Err(err) => { return Err(err); /*panic!( "An error occurred, terminating connection {}\n", err );*/ }
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
                    Err(err) => { return Err(err); /*panic!("An error occurred, terminating connection");*/ }
                    }
            }}
            // println!("{:?}", res_bytes);
            Ok(bytes_to_str(&res_bytes))
        }
        
        /// Responce formater
        // fn response_decoder(&mut self, responce: &str, attributes: Option<&Vec<String>>, key_values: &Vec<String>) -> Option::<Vec<HashMap::<String, String>>> { // temporary solution
        fn response_decoder(&mut self, responce: &str, query: &Queries) -> Result::<Vec::<String>, String> { // temporary solution
            let mut res         = Vec::<String>::new();
            let mut landfill    = responce.split("\n");
            let     fst_value   = landfill.nth(0).unwrap();

            if  &fst_value.len() >= &5usize && &fst_value[..5] == "!done"{
                return Err(format!("{}Empty message recieved{}: '!done' from {}{}{} in command {}", color::Fg(color::LightYellow), color::Fg(color::Reset), color::Fg(color::LightCyan),self.address, color::Fg(color::Reset), query.command));
            }if &fst_value.len() >= &5usize && &fst_value[..5] == "!trap" || &fst_value.len() >= &6usize && &fst_value[..6] == "!fatal" {
                // panic!("Here is an error during parsing because of invalid responce {:?}", landfill);
                return Err(format!("{}{}{} responce from {}{}{} in command {}", color::Fg(color::LightRed), fst_value, color::Fg(color::Reset), color::Fg(color::LightCyan),self.address, color::Fg(color::Reset), query.command));
            }
            let mut res_keys        = HashMap::new();
            let mut res_values      = HashMap::new();
            for piece in landfill{
                {
                    if &piece[..] == "!re"{
                        self.web_responce_formater(query, &res_keys, &res_values, &mut res);

                        // res_keys = HashMap::new();
                        res_keys = HashMap::new();

                        continue;
                    }else if piece.contains("=") {
                        let mut key = piece.split("=");
                        let (key, value) = (key.nth(0).unwrap(), key.nth(0).unwrap());
                        let key_formated = key.replace("-", "_");
                        if query.split_character != None && query.split_attributes.as_ref().unwrap_or(&Vec::new()).contains(&key.to_string()) && value.contains(query.split_character.as_ref().unwrap()){
                            let value = value.split(query.split_character.as_ref().unwrap()).collect::<Vec<&str>>();
                            match query.graph_targets.as_ref(){
                                Some(val) => {
                                        if val.contains(&key.to_string()){
                                            for i in 0..value.len(){ if value[i] != "" {
                                                res_values.insert(format!("{}_{}", key_formated, i), String::from(value[i]));
                                            }}
                                        }
                                    },
                                None => ()/* println!("Error message from router") */
                            }
                            match &query.attributes{
                                Some(val) => {
                                    if val.contains(&key.to_string()){
                                        for i in 0..value.len(){ if value[i] != "" {
                                            res_keys.insert(format!("{}_{}", key_formated, i), String::from(value[i]));
                                        }}
                                    }
                                },
                                None => { for i in 0..value.len(){ if value[i] != "" { res_keys.insert(format!("{}_{}", key, i), String::from(value[i])); } } }
                            }
                        }else{
                            match query.graph_targets.as_ref(){
                                Some(val) => {
                                        if val.contains(&key.to_string()){
                                            res_values.insert(String::from(&key_formated), String::from(value));
                                        }
                                    },
                                None => ()/* println!("Error message from router") */
                            }
                            match &query.attributes{
                                Some(val) => {
                                    if val.contains(&key.to_string()){
                                        res_keys.insert(key_formated, String::from(value));
                                    }
                                },
                                None => { res_keys.insert(key_formated, String::from(value)); }
                            }
                        }
                    }
                }
            }
            self.web_responce_formater(query, &res_keys, &res_values, &mut res);
            

            Ok(res)
        }

        /// Converts keys and values from `response_decoder` into a result string vector
        fn web_responce_formater(&self, query: &Queries, res_keys: &HashMap<String, String>, res_values: &HashMap<String, String>, res: &mut Vec<String>) {
            if res_values.len() == 0{
                res.push(
                    String::from(format!("miktik_{}{{routerboard_address=\"{}\"{}}} 0", query.name, self.address.to_string(),
                            (|| -> String {  
                                let mut res = String::new(); 
                                for (key, value) in res_keys {
                                    res += &format!(", {}_{}=\"{}\"", query.name, key, value); 
                                }
                                res
                            })()
                        ))
                );
            }else{
                for (key, value) in res_values{
                    res.push(
                        String::from(format!("miktik_{}_{}{{routerboard_address=\"{}\"{}}} {}", query.name, key.replace("-", "_"), self.address.to_string(),
                            (|| -> String {  
                                let mut res = String::new(); 
                                for (key, value) in res_keys {
                                    res += &format!(", {}_{}=\"{}\"", query.name, key, value); 
                                }
                                res
                            })(),
                            value.to_string() 
                        ))
                    );
                }
            }
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
        pub fn tell(&mut self, lines: &Vec::<String>, verbose: bool, _attributes: Option<&Vec<String>>) -> Result<String, io::Error>{//sender: &mut [net::TcpStream]
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            text.push(0);
            if self.secured { (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap(); } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = match self.reader(){
                Ok(val) => val,
                Err(err) => return Err(err)
            };
            if verbose == true{
                println!(">> {}", &output);
            }
            Ok(output)
        }

        /// Sends commands from list to routerboard after [Login] has been perforned
        /// 
        /// [Login]: login
        pub fn tell_get(&mut self, lines: &Vec::<String>, verbose: bool, query: &Queries, container: &mut Vec<String>) -> Result<(), ConnectionError>{//sender: &mut [net::TcpStream]
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            match &query.query{
                Some(val) => { for l in val { for x in hexer(l.as_bytes(), false) { text.push(x); } } }
                None => ()
            }
            text.push(0);
            if self.secured { (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap(); } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = match self.reader(){
                Ok(val) => val,
                Err(err) => return Err(ConnectionError::IoError(err))
            };
            let hash_res = self.response_decoder(&output[..], query);
            if verbose == true{
                match &hash_res{
                    Ok(val) => println!(">> {:#?}", val),
                    Err(msg) => eprintln!("{}Error{}: {}", color::Fg(color::LightRed), color::Fg(color::Reset), msg)
                }
            }

            match hash_res{
                Ok(mut value) => { container.append(&mut value); return Ok(()); },
                Err(msg) => Err(ConnectionError::ResponceError(msg))
            }
        }
        
        /// Executes commands from list runs web server with metrics to be reserved by prometheus
        // pub async fn queries_teller(&mut self, queries: Commands, verbosibility: bool, uri: String, port: u32, ) -> Vec::<HashMap<String, String>> {
        pub async fn queries_teller(connections: &mut Vec::<Connector>, queries_file: String, verbosibility: bool, uri: String, port: u32, ) -> Vec::<String> {
            let mut metrics = Vec::<String>::new();    
            let server = tiny_http::Server::http(format!("{}:{}", uri, port)).unwrap();
            let mut queries: Commands = type_reader(&queries_file);
            let mut reconnect_candidates = Vec::new();

            println!("{}Starting listening{} on: {}http://{}:{}{}", color::Fg(color::LightGreen), color::Fg(color::Reset), color::Fg(color::LightCyan), uri, port, color::Fg(color::Reset));

            loop{

                let request = match server.recv() {
                    Ok(rq) => { /* println!("{:?}", rq); */ rq },
                    Err(e) => { eprintln!("error: {}", e); break }
                };

                match request.url() {
                    "/metrics" => {
                        println!("{}{:?}{}:\twent to metrics page", style::Bold, request, style::Reset);

                        // adding commands output
                        for i in 0..connections.len(){
                            if !reconnect_candidates.contains(&i){
                                for command in &queries.commands{
                                    // println!("{}", command.command);
                
                                    match connections[i].tell_get( &vec![ command.command.to_string() ], verbosibility, command, &mut metrics ){
                                        Ok(_) => (),
                                        Err(err) => {
                                            match err{
                                                ConnectionError::ResponceError(msg) => eprintln!("{}", msg),
                                                ConnectionError::IoError(_err_) => { reconnect_candidates.push(i); eprintln!("Conneciton eror: {:?}", _err_); break; }
                                            }
                                        }
                                    }
                                }
                            }
                        }
        
                        let mut res = "".to_owned();
        
                        for value in &metrics{
                                res += &format!("{}\n", value);
                        }
        
                        let response = tiny_http::Response::from_string(res);
                        let _ = request.respond(response);
                        metrics = Vec::<String>::new();
                    },
                    "/" => { 
                        println!("{}{:?}{}: console home is here", style::Bold, request, style::Reset);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/index.html").unwrap())){
                            Ok(_) => (),
                            Err(e) => eprintln!("{}Error{} happened: {}", color::Fg(color::LightRed), color::Fg(color::Reset), e)
                        }
                    },
                    "/imgs/gears.gif" => {
                        println!("{}{:?}{}:\tgear gif file is here", style::Bold, request, style::Reset);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/images/gears.gif").unwrap())){
                            Ok(_) => (),
                            Err(e) => eprintln!("{}Error{} happened: {}", color::Fg(color::LightRed), color::Fg(color::Reset), e)
                        }
                    },
                    "/config/update" => {
                        println!("{}{:?}{}:\tupdating config", style::Bold, request, style::Reset);
                        queries = type_reader(&queries_file);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/reload_config.html").unwrap())){
                            Ok(_) => (),
                            Err(e) => eprintln!("{}Error{} happened: {}", color::Fg(color::LightRed), color::Fg(color::Reset), e)
                        }
                    }
                    _   => {
                        println!("{}{:?}{}", style::Bold, request, style::Reset);
                    }
                }

                let mut removed = 0;
                for i in 0..reconnect_candidates.len(){
                    match connections[reconnect_candidates[i]].reconnect(verbosibility){
                        Ok(()) => { 
                            println!("Reconnected to {}", connections[reconnect_candidates[i-removed]].address);
                            if reconnect_candidates.len() != 0{
                                reconnect_candidates.remove(i - removed);
                                removed += 1;
                            }
                        },
                        Err(err) => { 
                            match err {
                                ConnectionError::IoError(io_err) => eprintln!("Connection error during reconnection: {}", io_err),
                                ConnectionError::ResponceError(msg) => eprintln!("Resonce error during reconnection: {}", msg)
                            }
                        }
                    };
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
    // fn date_array_to_duration(time: [u8; 4]) -> Duration {
    //     Duration::new( (((( (time[0] as u64) * 24 )+ (time[1] as u64) * 60 )+ (time[2] as u64) * 60 ) + (time[3] as u64)) as u64, 0)
    // }

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