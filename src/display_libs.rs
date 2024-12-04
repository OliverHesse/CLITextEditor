use crossterm::style::{Attribute, Attributes, Color};

use std::{ fmt::format, thread::current};

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
    fn len(&self)->usize{
        let mut len:usize = 0;
        for i in 0..self.text.len(){
            len += self.text[i].len();
        }
        return len;
    }

}