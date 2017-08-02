extern crate irc;

use std::sync::Arc;
use std::thread;
use std::default::Default;
use irc::client::prelude::*;
use std::io;

fn main() {
    let config = create_config("program", "irc.mozilla.org", vec!["#mini, #rust".to_string()]);
    let data = Arc::new(IrcServer::from_config(config).unwrap());
    let server = data.clone();

    server.identify().unwrap_or_else(|err| {
            panic!("Error: {}", err);
    });

    thread::spawn(move ||{//Thread that handles receiving stuff
        server.for_each_incoming(|message| {
            match message.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    println!("{}: {}", target, msg);
                }
                _ => (),
            }
        }).unwrap_or_else(|err| {
                panic!("Error: {}", err);
        });
    });

    let server = data.clone();

    loop{ //Thread that draws and sends messages
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        server.send_privmsg("#mini", &guess).unwrap_or_else(|err| {
            panic!("Error Sending Message: {}", err);
        });
    }
}

fn create_config(nickname: &str, server: &str, channels: Vec<String>) -> Config{
    Config {
        nickname: Some(nickname.to_owned()),
        server: Some(server.to_owned()),
        channels: Some(channels),
        ..Default::default()
    }
}
