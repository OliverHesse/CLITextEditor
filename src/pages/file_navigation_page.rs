use crossterm::{cursor::{Hide, MoveTo, Show}, event::{Event, KeyCode, KeyEventKind, KeyEventState}, style::Print, terminal::{Clear, ClearType}, QueueableCommand};

use crate::{app::{self, App, AppAction, AppData}, page_libs::{PageCore, PageData}};
use std::{env::{self, current_dir}, io::{self, Write}, path::PathBuf};
pub struct FileNavigation{
    pub page_data:PageData,
  
    pub current_line:u16,
    pub current_column:u16,
    pub current_file_path:PathBuf,
    pub current_input_buffer:String,
}
impl FileNavigation{
    fn path_len_u16(&self)->u16{
        let curr_dir = &self.current_file_path;
        let len_u16: u16 = curr_dir.to_str().unwrap().len().try_into().unwrap_or_else(|_| {
            panic!("String is too long to fit into a u16");
        
        });
        return len_u16
    }
    fn path_len_usize(&self)->usize{
        let curr_dir = &self.current_file_path;
        let len_usize: usize = curr_dir.to_str().unwrap().len();
        return len_usize
    }
}
impl PageCore for FileNavigation{
    fn get_page_data(&self)->Option<PageData> {
        return Some(self.page_data.clone());
    }
    fn initial_draw(&mut self,app_data:AppData) ->AppAction{
        
        if let Ok(current_dir) = env::current_dir() {
            // Get the root component (drive on Windows, / on Linux)
            if let Some(root) = current_dir.components().next() {
                if let Some(root_str) = root.as_os_str().to_str() {
                    println!("Current root component: {}", root_str);
                    let len_u16: u16 = root_str.len().try_into().unwrap_or_else(|_| {
                        panic!("String is too long to fit into a u16");
        
                    });
                    self.current_column =len_u16+1;
                    self.current_file_path.push(root_str);
                    self.draw(app_data);
                }
            }
        } else {
            println!("Could not get the current directory");
        }
        AppAction::Nothing
        
    }
    fn draw(&self,app_data:AppData){
        //TODO left and right arrow key movement
        //TODO del
        //TODO undo-redo
        //TODO allow for paste
        //TODO enter
        let mut stdout = io::stdout();
        if let Some(path) = self.current_file_path.to_str(){
            let len_u16: u16 = path.len().try_into().unwrap_or_else(|_| {
                panic!("String is too long to fit into a u16");

            });
            
            stdout.queue(Hide);
            stdout.queue(MoveTo(0,self.current_line));
            stdout.queue(Clear(ClearType::CurrentLine));
            stdout.queue(Print(path.clone().to_owned()+">"+self.current_input_buffer.as_str()));
            stdout.queue(MoveTo(self.current_column,self.current_line));
            stdout.queue(Show);
        }else{
            println!("path does not exist");
        }
        stdout.flush();
       
    }
    fn run(&mut self,iter_event:crossterm::event::Event,app_data:AppData)->AppAction{
        match iter_event{
            Event::Key(e)=>{
                match e.code{
                    KeyCode::Left=>{
                        let curr_dir = &self.current_file_path;
                        let len_u16: u16 = curr_dir.to_str().unwrap().len().try_into().unwrap_or_else(|_| {
                            panic!("String is too long to fit into a u16");
                        
                        });
                        if e.kind == KeyEventKind::Press && self.current_column > self.path_len_u16()+1{
                            self.current_column -= 1;
                            self.draw(app_data);
                        }
                    },
                    KeyCode::Right=>{
                       
                        if e.kind == KeyEventKind::Press && usize::from(self.current_column) <= self.current_input_buffer.len()+self.path_len_usize(){
                            self.current_column += 1;
                            self.draw(app_data);
                        }
                    },
                    KeyCode::Char(ch)=>{

                        if e.kind == KeyEventKind::Press{
                            
                            self.current_input_buffer.insert(usize::from(self.current_column-self.path_len_u16()-1), ch);
                            //self.current_input_buffer.push(ch);
                            self.current_column+=1;
                            self.draw(app_data);
                    }
                    },
                    KeyCode::Backspace=>{
                        if e.kind == KeyEventKind::Press{
                            let in_i16:i16 = (self.current_column-self.path_len_u16()) as i16 -2;
                            if in_i16 >= 0{
                                let index_to_remove =usize::from(self.current_column-self.path_len_u16()-2);
                                if index_to_remove >= 0 && self.current_input_buffer.len() != 0 {
                                    self.current_input_buffer.remove(usize::from(self.current_column-self.path_len_u16()-2));
                                    self.current_column -= 1;
                                    self.draw(app_data)
                                }
                            }
                            
                       
                        }
                    },
                    KeyCode::Enter=>{},
                    _=>{}
                }
            },
            _=>{}
        }
       AppAction::Nothing
    }
}