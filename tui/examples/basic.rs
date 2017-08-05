extern crate cannon;

use cannon::{color, Console, ConsoleSize};

const MESSAGE_LN1: &'static str = "♦♦♦ Press Escape to Quit ♦♦♦";
const MESSAGE_LN2: &'static str = "♦♦♦ Cannon was created for LudumDare ♦♦♦";
const MESSAGE_LN3: &'static str = "♦♦♦ Github:  https://github.com/DevOrc/cannon ♦♦♦";

fn main(){
    let mut console = Console::new();
    let mut size = console.get_console_size();

    console.set_color(color::BLACK, color::LIGHT_GRAY);
    console.clear_screen();
    draw(&mut console, &size);

    'main: loop{
        if size != console.get_console_size(){
            console.set_color(color::BLACK, color::LIGHT_GRAY);
            console.clear_screen();
            size = console.get_console_size();
            draw(&mut console, &size);
        }

        let input =  console.poll_input();

        if let Some(i) = input{
            match i.EventType{
                1 => {
                        if key_is_escape(i.Event){
                            break 'main;
                        }
                        ()
                    }
                _ => (),
            }
        }
    }
}

fn key_is_escape(event: [u32;4]) -> bool{
    use cannon::input;

    //We don't care about releases
    if event[0] == 1{
        return false;
    }

    if let Some(key) = input::num_to_key(event[2]){
        if key == input::Key::Escape{
            return true;
        }
    }
    false
}


fn draw(console: &mut Console, size: &ConsoleSize){
    //Menu Bar
    console.set_color(color::RED, color::DARK_BLUE);
    for x in 0..size.width{
        console.write_character(x, size.height - 8, 32);
    }

    //Menu
    console.set_color(color::BLACK, color::DARK_GRAY);
    for y in 0..8 {
        for x in 0..size.width{
            console.write_character(x, size.height - y, 32);//18
        }
    }

    /*
    * █   █
    *
    *█     █
    * █████
    */

    //Draw Smiley Face
    let center_x = size.width / 2;
    let center_y = size.height / 2;

    //Draw Mouth
    console.set_color(color::BLACK, color::YELLOW);
    console.write_character(center_x - 3, center_y - 1, 32);
    for x in 0..4 {
        console.write_character(x + center_x - 2, center_y, 32);
    }
    console.write_character(center_x + 2, center_y - 1, 32);

    //Draw Eyes
    console.write_character(center_x - 2, center_y - 3, 32);
    console.write_character(center_x + 1, center_y - 3, 32);


    //Write the message TODO: Improve with a function/for loop
    console.set_color(color::YELLOW, color::DARK_GRAY);
    let chars: Vec<char> = MESSAGE_LN1.chars().collect();
    console.set_cursor_position((size.width / 2) - ((chars.len() / 2) as i16), size.height - 7);
    console.write(MESSAGE_LN1);

    let chars: Vec<char> = MESSAGE_LN2.chars().collect();
    console.set_cursor_position((size.width / 2) - ((chars.len() / 2) as i16), size.height - 6);
    console.write(MESSAGE_LN2);

    let chars: Vec<char> = MESSAGE_LN3.chars().collect();
    console.set_cursor_position((size.width / 2) - ((chars.len() / 2) as i16), size.height - 5);
    console.write(MESSAGE_LN3);

    //Finally Draw Screen size
    console.set_cursor_position(0, 0);
    console.set_color(color::RED, color::LIGHT_GRAY);
    console.write(&format!("Console Size: ({}, {})", size.width, size.height));
}
