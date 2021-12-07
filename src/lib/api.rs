use std::net::TcpListener;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::TcpStream;
use std::io::Write;
use crate::store_api::Store;
enum Command{
   Ping, Set(String, String), Delete(String), Get(String), Update(String, String), CommandNotRecognized
}

pub struct Api {
    socket: TcpListener,
    store: Store
}

impl Api {

    pub fn new(address: &str) -> Self {
        Api {
          socket: TcpListener::bind(address).unwrap(),
          store: Store::new()
        }
    }
    
    fn parse(message: &str) -> Command {
        println!("{}", message);
        if message.contains("PING") {
            Command::Ping
        } else if message.contains("SET") {
            let vector = message.split_whitespace().collect::<Vec<&str>>();
            Command::Set(vector[1].to_owned().to_string(),vector[2].to_owned().to_string())
        } else if message.contains("DELETE") {
            let vector = message.split_whitespace().collect::<Vec<&str>>();
            Command::Delete(vector[1].to_owned().to_string())
        } else if message.contains("GET") {
            let vector = message.split_whitespace().collect::<Vec<&str>>();
            Command::Get(vector[1].to_owned().to_string())
        } else if message.contains("UPDATE") {
            let vector = message.split_whitespace().collect::<Vec<&str>>();
            Command::Update(vector[1].to_owned().to_string(),vector[2].to_owned().to_string())
        } else {
            Command::CommandNotRecognized
        }
    }

    fn handle(&self, writer: &mut BufWriter<&TcpStream>, message: &str){
        let command = Api::parse(&message.replace("\n",""));
        match command {
            Command::Ping => {
                writer.write("PONG".as_bytes()).unwrap();
            },
            Command::Set(key, value) => {
                self.store.insert(&key, &value)
            },
            Command::Delete(key) => {
                self.store.remove(&key);
                writer.write(key.as_bytes()).unwrap();
            },
            Command::Get(key) => {
                if let Some(value) = &self.store.get(&key){
                    writer.write(value.as_bytes()).expect("Unexpected error.");
                } else {
                    writer.write("0".as_bytes()).expect("Unexpected error.");
                }
            },
            Command::Update(key, new_value) => {
                self.store.remove(&key);
                self.store.insert(&key, &new_value);
            },
            _ => {
                writer.write("Not Recognized".as_bytes()).unwrap();
            },
        }
    }
    

    pub fn poll(&self){
      for stream in self.socket.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                let mut buffer = Vec::new();
                let _ = reader.read_until(b'\n', &mut buffer);
                let req_as_string = String::from_utf8(buffer).expect("Error parsing request").to_owned();
                self.handle(&mut writer, &req_as_string);
            }
            Err(_) => { 
                println!("Unexpected Error")
            }
        }
      }
    }
}

