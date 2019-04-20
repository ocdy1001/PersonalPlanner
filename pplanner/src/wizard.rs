use std::collections::VecDeque;

use super::astr;
use super::data;
use super::conz;
use super::conz::PrinterFunctions;
use super::astr::*;

pub enum InputType{
    Text,
    DateTime,
    Bool,
}

pub enum PromptType{
    Reprompt,   //ask until good or user cancels -> push good val or error
    Once,       //ask one time -> push good val or error
    Partial,    //ask one time -> push good val or push nothing
}

struct Field{
    field_type: InputType,
    prompt_msg: astr::Astr,
    prompt_type: PromptType,
}

pub struct FieldVec{
    vec: Vec<Field>,
}

impl FieldVec{
    pub fn new() -> Self{
        FieldVec{
            vec: Vec::new(),
        }
    }

    pub fn add(&mut self, field_type: InputType, prompt_msg: astr::Astr, prompt_type: PromptType){
        self.vec.push(Field{
            field_type: field_type,
            prompt_msg: prompt_msg,
            prompt_type: prompt_type,
        });
    }

    pub fn execute(&self) -> Result<WizardRes,()>{
        let mut texts: VecDeque<astr::Astr> = VecDeque::new();
        let mut datetimes: VecDeque<data::DT> = VecDeque::new();
        let mut bools: VecDeque<bool> = VecDeque::new();
        for instr in &self.vec{
            loop {
                let is_ok = match instr.field_type{
                    InputType::Text => Self::handle_text(&mut texts, &instr),
                    InputType::DateTime => Self::handle_datetime(&mut datetimes, &instr),
                    InputType::Bool => Self::handle_bool(&mut bools, &instr),
                };
                if is_ok {break;}
                match instr.prompt_type{
                    PromptType::Once =>{
                        conz::printer().println_type(&"Fail: could not parse.", conz::MsgType::Error);
                        return Err(());
                    }
                    PromptType::Reprompt =>{
                        let redo = conz::prompt("Could not parse, try again? */n: ");
                        if redo == "n" { return Err(()); }
                    }
                    PromptType::Partial =>{
                        break;
                    }
                }
            }
        }
        let res = WizardRes::new(texts, datetimes, bools);
        return Ok(res);
    }

    fn handle_text(texts: &mut VecDeque<astr::Astr>, field: &Field) -> bool{
        let line = conz::prompt(&field.prompt_msg.to_string()).to_astr();
        texts.push_back(line);
        return true;
    }

    fn handle_datetime(datetimes: &mut VecDeque<data::DT>, field: &Field) -> bool{
        let lines = astr::from_str(&conz::prompt(&field.prompt_msg.to_string())).split_str(&astr::astr_whitespace());
        if lines.len() != 2 { return false; }
        let tri0 = data::parse_dmy_or_hms(&lines[0]);
        let tri1 = data::parse_dmy_or_hms(&lines[1]);
        if tri0.is_err() { return false; }
        if tri1.is_err() { return false; }
        let dt1 = data::DT::make_datetime(tri1.unwrap(), tri0.unwrap());
        if dt1.is_err() { return false; }
        datetimes.push_back(dt1.unwrap());
        return true;
    }

    fn handle_bool(bools: &mut VecDeque<bool>, field: &Field) -> bool{
        conz::read_bool(&field.prompt_msg.to_string())
    }
}

pub struct WizardRes{
    all_text: VecDeque<astr::Astr>,
    all_datetime: VecDeque<data::DT>,
    all_bool: VecDeque<bool>,
}

impl WizardRes{
    pub fn new(text: VecDeque<astr::Astr>, dt: VecDeque<data::DT>, bools: VecDeque<bool>) -> Self{
        WizardRes{
            all_text: text,
            all_datetime: dt,
            all_bool: bools,
        }
    }

    pub fn extract_point(&mut self) -> Result<data::Point,()>{
        loop{
            if self.all_text.len() < 1 {break;}
            if self.all_datetime.len() < 1 {break;}
            let dt_res = self.all_datetime.pop_front();
            if dt_res.is_none() {break;}
            let title_res = self.all_text.pop_front();
            if title_res.is_none() {break;}
            let isdead_res = self.all_text.pop_front();
            if isdead_res.is_none() {break;}
            let ret = data::Point::new(dt_res.unwrap(), title_res.unwrap(), isdead_res.unwrap());
            return Ok(ret);
        }
        conz::printer().println_type(&"Error: could not build point.", conz::MsgType::Error);
        return Err(());
    }

    pub fn get_text(&mut self) -> Result<astr::Astr,()>{
        let res = self.all_text.pop_front();
        if res.is_none() {return Err(());}
        return Ok(res.unwrap());
    }

    pub fn get_dt(&mut self) -> Result<data::DT,()>{
        let res = self.all_datetime.pop_front();
        if res.is_none() {return Err(());}
        return Ok(res.unwrap());
    }

    pub fn get_bool(&mut self) -> Result<bool,()>{
        let res = self.all_bool.pop_front();
        if res.is_none() {return Err(());}
        return Ok(res.unwrap());
    }
}
