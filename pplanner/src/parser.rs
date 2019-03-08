use std::collections::HashMap;

use termcolor::{ Color };

use super::conz;
use super::astr;

pub struct Parser {
    funcs: HashMap<astr::Astr, fn(&mut conz::Printer, astr::AstrVec)>,
    printer: conz::Printer,
}

impl Parser {
    pub fn new(printer: conz::Printer) -> Parser {
        let mut funcs: HashMap<astr::Astr, fn(&mut conz::Printer, astr::AstrVec)> = HashMap::new();
        funcs.insert(astr::from_str("now"), commands::now);
        return Parser {
            funcs,
            printer,
        }
    }

    pub fn start_loop(&mut self) {
        self.printer.println_type("Henlo Fren!", conz::MsgType::Prompt);
        self.printer.println_type("pplanner: a ascii cli time management tool.", conz::MsgType::Prompt);
        self.printer.println_type("Made by Cody Bloemhard.", conz::MsgType::Prompt);
        loop{
            let x = conz::prompt(&mut self.printer, "cmd > ");
            let y = x.as_ref();
            match y {
                "q" => break,
                "quit" => break,
                _ => {
                    let found_cmd = self.parse_and_run(y);
                    if found_cmd { continue; }
                    self.printer.println_error("Error: Command not found: \"", y, "\"!");
                }
            }
        }
        self.printer.println_color("Bye!", Color::Cyan);
    }

    fn parse_and_run(&mut self, line: &str) -> bool{
        let command = astr::split(&astr::from_str(line), &astr::from_str(" \n\t"));
        let search_res = self.funcs.get(&command[0]);
        match search_res {
            None => return false,
            Some(x) => x(& mut self.printer, command),
        }
        return true;
    }
}

mod commands {
    use super::super::conz;
    use super::super::data;
    use super::super::astr;

    pub fn now(printer: &mut conz::Printer, _command: astr::AstrVec){
        let dt = data::DT::new();
        printer.println_type(dt.str_datetime().as_ref(), conz::MsgType::Value);

        let triplet = data::parse_dmy_or_hms(&_command[1]);
        match triplet{
            Ok(x) => printer.println_type(format!("{}!{}!{}", x.0, x.1, x.2).as_ref(), conz::MsgType::Highlight),
            _ => return,
        }
    }
}
