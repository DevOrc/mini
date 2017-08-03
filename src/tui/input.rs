extern crate cannon;

use self::cannon::{Console};
use self::cannon::input::*;
use std::thread;
use std::sync::mpsc::{channel, Receiver};

pub struct InputSystem{
    rx: Receiver<Key>
}

impl InputSystem{
    pub fn poll(&self) -> Option<Key>{
        match self.rx.try_recv() {
            Err(_) => None,
            Ok(k) => Some(k)
        }
    }
}

pub fn init() -> InputSystem{
    let (tx, rx) = channel();

    thread::spawn(move ||{
        let mut console = Console::new();
        console.set_should_cls(false);

        loop{
            let input =  console.poll_input();

            if let Some(i) = input{
                let key_opt = match i.EventType{
                    1 => to_key(i.Event),
                    _ => None,
                };

                if let Some(key) = key_opt{
                    tx.send(key).unwrap_or_else(|err| {
                        panic!("Input System Channel Error: {}", err);
                    });
                }
            }
        }
    });

    InputSystem {rx: rx}
}

fn to_key(event: [u32;4]) -> Option<Key>{
    if event[0] == 0{
        return None;
    }

    num_to_key(event[2])
}
