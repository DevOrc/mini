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
    input_chars: Vec<char>
}

impl Tui{
    pub fn draw_input_bar(&mut self){
        let size = self.console.get_console_size();
        let s: String = self.input_chars.clone().into_iter().collect();

        self.console.set_color(color::DARK_BLUE, color::DARK_BLUE);
        rect(&mut self.console, 0, size.height - INPUT_BAR_HEIGHT,
            size.width + 1, size.height - INPUT_BAR_HEIGHT);
        self.reset_position();

        self.console.set_color(color::BLACK, color::LIGHT_GRAY);
        rect(&mut self.console, 0, size.height - INPUT_BAR_HEIGHT + 1,
            size.width + 1, size.height - INPUT_BAR_HEIGHT + 1);
        self.console.write(&s);
    }

    fn calc_input_position(&self) -> ConsolePosition{
        let size = self.console.get_console_size();
        ConsolePosition{x: 0, y: size.height - INPUT_BAR_HEIGHT + 1}
    }

    pub fn add_message(&mut self, string: &str){
        self.output_lines.push_back(string.into());

        if self.output_lines.len() > self.output_area_height() as usize{
            self.output_lines.pop_front();
        }

        self.draw_output(true);
    }

    fn output_area_height(&self) -> i16 {
        let size = self.console.get_console_size();
        size.height - INPUT_BAR_HEIGHT
    }

    fn write_message(&mut self, string: &str){
        self.console.set_cursor_position(0, self.output_line);
        self.output_line += 1;
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
        let pos = self.calc_input_position();
        self.console.set_cursor_position(pos.x, pos.y);
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
                    self.input_chars.push(c);
                    self.draw_input_bar();
                    None
                },
                Key::Num(n) => {
                    self.input_chars.push((n + 48) as char);
                    self.draw_input_bar();
                    None
                },
                Key::Enter => {
                     let msg: String = self.input_chars.clone().into_iter().collect();
                     self.add_message(&format!("You: {}", &msg));
                     self.input_chars = Vec::new();
                     self.draw_input_bar();
                     Some(TuiEvent::SendMsg(msg))
                },
                Key::Backspace => {
                    self.input_chars.pop();
                    self.draw_input_bar();
                    None
                },
                _ => None
            };
        }

        None
    }
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
        input_chars: Vec::new(),
        output_lines: VecDeque::new()
    };
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
