use crossterm::{style::{Attribute, Attributes, Color, Print}, QueueableCommand};

use std::{ fmt::format, io, process::Output, thread::current};

//stores a string with attributes attached to it
#[derive(Clone,Debug)]
pub struct StyledSpan{
    pub span_text:Vec<char>,
    pub background_colour:crossterm::style::Color,
    pub colour:crossterm::style::Color,
    pub attributes:crossterm::style::Attributes,
    
}
impl StyledSpan{
    fn eq_style(&self, other: &Self)->bool{
        (self.background_colour == other.background_colour) && (self.colour == other.colour)&&(self.attributes == other.attributes)
    
    }  
    fn len(&self)->usize{
        return self.span_text.len();
    }
}

//stores a string of lines
#[derive(Clone,Debug)]
pub struct Line{
    pub text:Vec<StyledSpan>,
}

impl Line{
    pub fn queue_line(&self ,stdout: &mut io::Stdout){
        let mut final_string = String::new();
        for i in 0..self.text.len(){
            final_string += self.text[i].span_text.iter().collect::<String>().as_str();
            
        }
        stdout.queue(Print(final_string));
    }
    fn len(&self)->usize{
        let mut len:usize = 0;
        for i in 0..self.text.len(){
            len += self.text[i].len();
        }
        return len;
    }

}