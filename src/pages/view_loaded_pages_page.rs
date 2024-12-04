use std::io::{self, Write};

use crossterm::{cursor::{Hide, MoveTo}, event::Event, style::Print, QueueableCommand};

use crate::{app::{App, AppAction, AppData}, display_libs::Line, page_libs::{PageCore, PageData}};


//for selected line, highlight white and make text black

pub struct ViewLoadedPages{
    page_data:PageData,
    
    current_line:u16,
    text:Vec<Line>
}

impl PageCore for ViewLoadedPages{
    /*fn initial_draw(&mut self,app_data:AppData)->AppAction{
        let mut stdout = io::stdout();
        let mut current_line:u16 = 0;
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,0));
        for n in all_pages{
            let wrapped_details = n.get_page_display_details();
            if wrapped_details.is_none(){
                match crossterm::queue!(
                    stdout,
                    Print(""),
                    
                ) {
                    Ok(_)=>{},
                    Err(e)=>{panic!("error here")}
                }
            } else {
                let details = wrapped_details.unwrap();
                match crossterm::queue!(
                    stdout,
                    MoveTo(current_line,0),
                    Print(details.page_name),
                    MoveTo(current_line,40),
                    Print(details.file_path),
                    
                    
                ){
                    Ok(_)=>{},
                    Err(e)=>{panic!("error here")}
                }
                
            }
            current_line+=1

        }
        stdout.flush();
        AppAction::Nothing
    }*/
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction {
        AppAction::Nothing
    }
}
