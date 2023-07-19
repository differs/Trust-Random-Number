use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct RandomGenerators {
    health: StdRng,
    stamina: StdRng,
    attack: StdRng,
    defense: StdRng,
}

impl RandomGenerators {
    fn new() -> Self {
        Self {
            health: StdRng::from_entropy(),
            stamina: StdRng::from_entropy(),
            attack: StdRng::from_entropy(),
            defense: StdRng::from_entropy(),
        }
    }

    fn generate(&mut self, name: &str) -> Option<i32> {
        match name {
            "health" => Some(self.health.gen_range(50..=100)),
            "stamina" => Some(self.stamina.gen_range(20..=50)),
            "attack" => Some(self.attack.gen_range(10..=30)),
            "defense" => Some(self.defense.gen_range(5..=20)),
            _ => None,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let generators = Arc::new(Mutex::new(RandomGenerators::new()));
    println!("Listening on port 8080...");

    for stream in listener.incoming() {
        let generators = generators.clone();
        let mut stream = stream.unwrap();

        thread::spawn(move || {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let response = match String::from_utf8(buffer.to_vec()) {
                Ok(request) => {
                    let request_line = request.lines().next().unwrap();
                    let parts: Vec<_> = request_line.split_whitespace().collect();
                    match (parts.get(0), parts.get(1)) {
                        (Some(&"GET"), Some(path)) => {
                            let name = path.trim_start_matches('/');
                            let mut generators = generators.lock().unwrap();
                            match generators.generate(name) {
                                Some(value) => format!("{}: {}", name, value),
                                None => "Invalid request".to_string(),
                            }
                        }
                        _ => {
                            "Invalid request".to_string()
                        }
                    }
                }
                Err(_) => "Invalid request".to_string(),
            };

            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
    }
}