extern crate cannon;

mod input;

use std::collections::VecDeque;
use self::input::*;
use self::cannon::*;
use self::cannon::input::Key;

///Includes the bar
const INPUT_BAR_HEIGHT: i16 = 4;

pub enum TuiEvent{
    Quit,
    SendMsg(String)
}

#[derive(Clone)]
struct ConsolePosition{
    x: i16,
    y: i16
}

pub struct Tui{
    console: Console,
    prev_size: ConsoleSize,
    input: InputSystem,
    output_line: i16,
    output_lines: VecDeque<String>,
    input_chars: Vec<char>,
    cursor_pos: ConsolePosition
}

impl Tui{
    pub fn draw_input_bar(&mut self){
        let size = self.console.get_console_size();
        let s: String = self.input_chars.clone().into_iter().collect();
        let pos = self.cursor_pos.clone();

        self.console.set_color(color::DARK_BLUE, color::DARK_BLUE);
        rect(&mut self.console, 0, size.height - INPUT_BAR_HEIGHT,
            size.width + 1, size.height - INPUT_BAR_HEIGHT);
        self.calc_input_position();

        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        rect(&mut self.console, 0, size.height - INPUT_BAR_HEIGHT + 1,
            size.width + 1, size.height);
        self.calc_input_position();
        self.reset_position();
        //self.add_message(&s);
        self.console.write(&s);
        self.cursor_pos = pos;
        self.reset_position();
    }

    fn calc_input_position(&mut self){
        let size = self.console.get_console_size();
        self.cursor_pos = ConsolePosition{x: 0, y: size.height - INPUT_BAR_HEIGHT + 1};
    }

    pub fn add_message(&mut self, string: &str){
        self.output_lines.push_back(string.into());

        let mut output_line = self.output_line + 1 + string_to_char_length(&string.to_string()) as i16 /
            self.console.get_console_size().width;

        while output_line > self.output_area_height(){
            let s = self.output_lines.pop_front();
            output_line -= 1 + string_to_char_length(&s.unwrap()) as i16 /
                self.console.get_console_size().width;
        }

        self.output_line = output_line;

        self.draw_output(true);
    }

    fn output_area_height(&self) -> i16 {
        let size = self.console.get_console_size();
        size.height - INPUT_BAR_HEIGHT
    }

    fn write_message(&mut self, string: &str){
        self.console.set_cursor_position(0, self.output_line);
        self.output_line += 1 + string_to_char_length(&string.to_string()) as i16 /
            self.console.get_console_size().width;
        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        self.console.write(string);
        self.reset_position();
    }

    pub fn draw_output(&mut self, redraw_background: bool){
        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        self.console.set_cursor_position(0, 0);
        self.output_line = 0;

        if redraw_background{
            let size = self.console.get_console_size();
            rect(&mut self.console, 0, 0, size.width + 1, size.height - INPUT_BAR_HEIGHT -1);
        }

        for line in &self.output_lines.clone() {
            self.write_message(line);
        }
    }

    pub fn redraw(&mut self){
        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        self.console.clear_screen();
        self.console.set_cursor_position(0, 0);

        self.draw_output(false);
        self.draw_input_bar();

        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        self.reset_position();
    }

    pub fn reset_position(&self){
        self.console.set_cursor_position(self.cursor_pos.x, self.cursor_pos.y);
    }

    fn check_cursor_pos(&mut self){
        if self.cursor_pos.x < 0{
            self.cursor_pos.x = 0;
        }
        if self.cursor_pos.x > self.input_chars.len() as i16{
            self.cursor_pos.x = self.input_chars.len() as i16; 
        }
    }

    pub fn move_cursor(&mut self, x: i16, y: i16){
        let pos = self.cursor_pos.clone();
        self.cursor_pos = ConsolePosition{x: pos.x + x, y: pos.y + y}
    }

    pub fn update(&mut self) -> Option<TuiEvent>{
        if self.prev_size != self.console.get_console_size(){
            self.redraw();
            self.prev_size = self.console.get_console_size();
        }

        if let Some(key) = self.input.poll(){
            return match key {
                Key::Escape => Some(TuiEvent::Quit),
                Key::Char(c) => {
                    if self.input_chars.len() >= self.cursor_pos.x as usize
                        && self.cursor_pos.x >= 0{
                        self.input_chars.insert(self.cursor_pos.x as usize, c);
                        self.move_cursor(1, 0);
                    }
                    self.draw_input_bar();
                    None
                },
                Key::Num(n) => {
                    self.input_chars.push((n + 48) as char);
                    self.draw_input_bar();
                    self.move_cursor(1, 0);
                    self.reset_position();
                    None
                },
                Key::Enter => {
                     let msg: String = self.input_chars.clone().into_iter().collect();
                     self.add_message(&format!("You: {}", &msg));
                     self.input_chars = Vec::new();
                     self.calc_input_position();
                     self.draw_input_bar();
                     Some(TuiEvent::SendMsg(msg))
                },
                Key::Backspace => {
                    if self.input_chars.len() > self.cursor_pos.x as usize - 1
                        && self.cursor_pos.x - 1 >= 0{
                        self.move_cursor(-1, 0);
                        self.input_chars.remove(self.cursor_pos.x as usize);
                    }
                    self.draw_input_bar();
                    None
                },
                Key::Left => {
                    self.move_cursor(-1, 0);
                    self.check_cursor_pos();
                    self.reset_position();
                    None
                }
                Key::Right => {
                    self.move_cursor(1, 0);
                    self.check_cursor_pos();
                    self.reset_position();
                    None
                }
                _ => None
            };
        }

        None
    }
}

fn string_to_char_length(input: &String) -> usize{
    input.chars().count()
}

pub fn init() -> Tui{
    let mut console = Console::new();
    let size = console.get_console_size();
    console.set_should_cls(true);

    let mut tui = Tui {
        output_line: 0,
        prev_size: size,
        console: console,
        input: input::init(),
        input_chars: Vec::with_capacity(5),
        output_lines: VecDeque::new(),
        cursor_pos: ConsolePosition{x:0, y:0}
    };
    tui.calc_input_position();
    tui.redraw();
    tui
}

fn rect(console: &mut Console, x1: i16, y1: i16, x2: i16, y2: i16){
    let width = x2 - x1;
    let height = y2 - y1;

    for x in 0..width + 1 {
        for y in 0..height + 1{
            console.write_character(x +x1, y + y1, 32);
        }
    }
}
