use std::io::{self, Write};

use crossterm::{cursor::{Hide, MoveTo}, event::{Event, KeyCode, KeyEvent, KeyEventKind}, style::{Color, Print, SetBackgroundColor}, terminal::{self, Clear, ClearType}, QueueableCommand};

use crate::{app::{App, AppAction, AppData}, display_libs::Line, page_libs::{PageCore, PageData}};


//for selected line, highlight white and make text black

pub struct ViewLoadedPages{
    pub page_data:PageData,
    
    pub current_line:u16,
    pub text:Vec<String>
}

impl ViewLoadedPages{
    pub fn reset_line(&self ,index:usize){
        let mut stdout = io::stdout();
        let (cols, rows) = terminal::size().unwrap();
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,self.current_line));
        stdout.queue(Clear(ClearType::CurrentLine));
        stdout.queue(SetBackgroundColor(Color::Black));
      

        stdout.queue(Print(self.text[self.current_line as usize].as_str()));
        for _ in 0..(usize::from(cols)-self.text[self.current_line as usize].len()){
            stdout.write_all(b" ").unwrap();
            
        }
        stdout.flush();
    }
    pub fn hover_current_line(&self){
        let mut stdout = io::stdout();
        let (cols, rows) = terminal::size().unwrap();
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,self.current_line));
        stdout.queue(Clear(ClearType::CurrentLine));
        stdout.queue(SetBackgroundColor(Color::DarkGrey));
      
     
        stdout.queue(Print(self.text[self.current_line as usize].as_str()));
        for _ in 0..(usize::from(cols)-self.text[self.current_line as usize].len()) {
            stdout.write_all(b" ").unwrap();
            
        }
        
        stdout.flush();
        

    }
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
           
            self.text.push(format!(
                "{}{}{}", 
                details.page_name.clone(),
                " ".repeat(40), // Create 40 spaces
                details.file_path.display()
            ));
            match crossterm::queue!(
                stdout,
                MoveTo(0,current_line),
                Print(self.text[self.text.len()-1].as_str()),
                
                
            ){
                Ok(_)=>{},
                Err(e)=>{panic!("error here")}
            }
                
            
            current_line+=1

        }
        stdout.flush();
        
        self.hover_current_line();
        AppAction::Nothing
    }
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction {
        //TODO make work with text larger than screen size(scrolling)
        match iter_event{
            Event::Key(e)=>{
                match e.code{
                    KeyCode::Up=>{
                        if e.kind == KeyEventKind::Press && self.current_line >0 && usize::from(self.current_line) < self.text.len(){
                            self.reset_line(self.current_line as usize);
                            self.current_line -=1;
                            self.hover_current_line();
                        }
                    },
                    KeyCode::Down=>{
                        
                        if e.kind == KeyEventKind::Press && usize::from(self.current_line) < self.text.len()-1{
                          
                            self.reset_line(self.current_line as usize);

                            self.current_line +=1;
                            self.hover_current_line();
                        } 
                    },
                    KeyCode::Backspace=>{},
                    KeyCode::Enter=>{},
                    _=>{}
                }
            }
            _=>{}
        }
        AppAction::Nothing
    }
}
