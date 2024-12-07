use std::io::{self, Write};
use std::env;
use std::fs;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal;
use crossterm::{cursor::{Hide, MoveTo, Show}, event::Event, style::Print, terminal::{Clear, ClearType}, QueueableCommand};

use crate::{app::{App, AppAction, AppData}, page_libs::{PageCore, PageData}};
//TODO add change stack with CTRL Z and CTRL Y
pub struct TextFilePage{
    pub page_data:PageData,
    pub text:Vec<Vec<char>>,
    pub current_line:u16,
    pub current_row:u16,
}
impl PageCore for TextFilePage{
    fn get_page_data(&self)->Option<PageData> {
        return Some(self.page_data.clone())
    }
    fn initial_draw(&mut self,app_data:AppData)->AppAction {
        let mut stdout = io::stdout();
        let (cols, rows) = terminal::size().unwrap();
        //so i dont reset anything if initial_draw is called again
        
        if self.text.is_empty() == false{

            self.draw(app_data);
            return AppAction::Nothing;
        }
        let contents = fs::read_to_string(self.page_data.file_path.as_path())
        .expect("Should have been able to read the file");
        let file_rows:Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
        let mut iter = file_rows.iter();
        
        stdout.queue(Hide);
        stdout.queue(SetBackgroundColor(Color::Black));
        for (i,row) in iter.enumerate(){
            let row_vec:Vec<char> = row.chars().collect();
            //TODO not super safe see what i can do about it
            if i<(rows as usize){
                stdout.queue(MoveTo(0,i.try_into().unwrap()));
                stdout.queue(Clear(ClearType::CurrentLine));
                stdout.queue(Print(row_vec.iter().collect::<String>()));
            }
            self.text.push(row_vec);

        }
        
        
        stdout.queue(MoveTo(0,0));

        stdout.queue(Show);
        stdout.flush();
        AppAction::Nothing
    }
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction {
        AppAction::Nothing
    }
    fn get_page_name(&self)->String{
        return self.page_data.file_name.clone();
    }
}