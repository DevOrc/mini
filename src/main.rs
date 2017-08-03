extern crate irc;

mod tui;

use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::default::Default;
use irc::client::prelude::*;
use self::tui::TuiEvent;

fn main() {
    let config = create_config(
        "program",
        "irc.mozilla.org",
        vec!["#mini, #rust".to_string()],
    );
    let data = Arc::new(IrcServer::from_config(config).unwrap());
    let server = data.clone();
    let (tx, rx) = channel();
    let mut tui = tui::init();

    server.identify().unwrap_or_else(|err| {
        panic!("Error: {}", err);
    });

    thread::spawn(move || {
        server.for_each_incoming(|message| {
            tx.send(message).unwrap_or_else(|err| {
                    panic!("Error: {}", err);
                });
            })
        .unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });
    });

    let server = data.clone();

    'main: loop {
        if let Some(event) = tui.update(){
            match event{
                TuiEvent::Quit => break 'main,
                TuiEvent::SendMsg(string) => {
                    server.send_privmsg("#mini", &string);
                    ()
                }

                _ => ()
            }
        };

        //Thread that draws and sends messages
        let message = match rx.try_recv() {
            Err(_) => None,
            Ok(c) => Some(c),
        };

        if let Some(m) = message {
            match m.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    tui.add_message(&format!("{}: {}", target, msg));
                }
                _ => (),
            }
        }
    }
}

fn create_config(nickname: &str, server: &str, channels: Vec<String>) -> Config {
    Config {
        nickname: Some(nickname.to_owned()),
        server: Some(server.to_owned()),
        channels: Some(channels),
        ..Default::default()
    }
}
