use std::io::{self, Write};

use crossterm::{cursor::{Hide, MoveTo}, event::Event, style::Print, QueueableCommand};

use crate::{app::{App, AppAction, AppData}, display_libs::Line, page_libs::{PageCore, PageData}};


//for selected line, highlight white and make text black

pub struct ViewLoadedPages{
    pub page_data:PageData,
    
    pub current_line:u16,
    pub text:Vec<Line>
}

impl PageCore for ViewLoadedPages{
    fn get_page_name(&self)->String{
        return self.page_data.page_name.clone();
    }
    fn get_page_data(&self)->Option<PageData> {
        return Some(self.page_data.clone());
    }
    fn initial_draw(&mut self,app_data:AppData)->AppAction{
        let mut stdout = io::stdout();
        let mut current_line:u16 = 0;
        let all_pages =app_data.pages_data;
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,0));
        for n in all_pages{
            let details = n.get_page_display_details();
           
            
            match crossterm::queue!(
                stdout,
                MoveTo(0,current_line),
                Print(details.page_name.clone()),
                MoveTo(40,current_line),
                Print(details.file_path.display()),
                
                
            ){
                Ok(_)=>{},
                Err(e)=>{panic!("error here")}
            }
                
            
            current_line+=1

        }
        stdout.flush();
        AppAction::Nothing
    }
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction {
        AppAction::Nothing
    }
}
