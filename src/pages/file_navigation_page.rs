use crossterm::{cursor::{Hide, MoveTo, Show}, event::{Event, KeyCode, KeyEventKind, KeyEventState}, style::Print, terminal::{self, Clear, ClearType}, QueueableCommand};

use crate::{app::{self, App, AppAction, AppData}, page_libs::{PageCore, PageData}};
use std::{env::{self, current_dir}, fs, io::{self, Write}, path::{Path, PathBuf}};
pub struct FileNavigation{
    pub page_data:PageData,
  
    pub current_line:u16,
    pub current_column:u16,
    pub current_file_path:PathBuf,
    pub current_input_buffer:String,
}
impl FileNavigation{
    fn get_page_name(&self)->String{
        return self.page_data.file_name.clone();
    }
    fn path_len_u16(&self)->u16{
        let curr_dir = &self.current_file_path;
        let len_u16: u16 = curr_dir.to_str().unwrap().len().try_into().unwrap_or_else(|_| {
            panic!("String is too long to fit into a u16");
        
        });
        return len_u16
    }
    fn draw_output(&mut self,output:String){
        let mut stdout = io::stdout();
        self.current_line += 1;
        if self.current_line >= terminal::size().unwrap().1-1{
            stdout.queue(terminal::ScrollUp(1));
        }
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,self.current_line));
        stdout.queue(Clear(ClearType::CurrentLine));
        stdout.queue(Print(output.as_str()));
        stdout.queue(MoveTo(self.current_column,self.current_line));
        stdout.queue(Show);
        
    
        stdout.flush();
    }

    fn reset_cursor(&mut self){
        let curr_dir = &self.current_file_path;
        let len_u16: u16 = curr_dir.to_str().unwrap().len().try_into().unwrap_or_else(|_| {
            panic!("String is too long to fit into a u16");
        
        });
        self.current_column =len_u16+1;
    }
    fn path_len_usize(&self)->usize{
        let curr_dir = &self.current_file_path;
        let len_usize: usize = curr_dir.to_str().unwrap().len();
        return len_usize
    }
    fn normalize_path(&self,path: &str) -> PathBuf {
        let path = Path::new(path);
        let normalized = fs::canonicalize(path).expect("Failed to canonicalize path");

        
        let normalized_str = normalized.to_str().expect("Failed to convert path to string");
        if normalized_str.starts_with(r"\\?\") {
            let stripped = &normalized_str[4..]; 
            Path::new(stripped).to_path_buf()  
        } else {
            normalized
        }
    }
    fn edit_input_buffer(&mut self){
        
        let mut stdout = io::stdout();
        if let Some(path) = self.current_file_path.to_str(){
            let len_u16: u16 = path.len().try_into().unwrap_or_else(|_| {
                panic!("String is too long to fit into a u16");

            });
            if self.current_line >= terminal::size().unwrap().1{
                //stdout.queue(terminal::ScrollUp(1));
            }
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
}
impl PageCore for FileNavigation{
    fn get_page_name(&self)->String {
        return self.page_data.page_name.clone();
    }
    fn get_page_data(&self)->Option<PageData> {
        return Some(self.page_data.clone());
    }

    fn initial_draw(&mut self,app_data:AppData) ->AppAction{
        self.current_column = 0;
        self.current_line = 0;
        if self.current_file_path.as_os_str().is_empty(){
            if let Ok(current_dir) = env::current_dir() {
                // Get the root component (drive on Windows, / on Linux)
                if let Some(root) = current_dir.components().next() {
                    if let Some(root_str) = root.as_os_str().to_str() {
                        
                        let len_u16: u16 = root_str.len().try_into().unwrap_or_else(|_| {
                            panic!("String is too long to fit into a u16");
            
                        });
                        self.current_column =len_u16+2;
                        let path_str = root_str.to_string()+r"\";
                        self.current_file_path.push(path_str);
                        self.draw(app_data);
                    }
                }
            } else {
                println!("Could not get the current directory");
            }
        }else{
            self.current_column = self.path_len_u16() + 1;
            self.draw(app_data);
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
            if self.current_line >= terminal::size().unwrap().1{
                stdout.queue(terminal::ScrollUp(1));
            }    
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
                        if e.kind == KeyEventKind::Press && self.current_column > self.path_len_u16()+1{
                            self.current_column -= 1;
                            self.edit_input_buffer();
                        }
                    },
                    KeyCode::Right=>{
                       
                        if e.kind == KeyEventKind::Press && usize::from(self.current_column) <= self.current_input_buffer.len()+self.path_len_usize(){
                            self.current_column += 1;
                            self.edit_input_buffer();
                        }
                    },
                    KeyCode::Char(ch)=>{

                        if e.kind == KeyEventKind::Press{
                            
                            self.current_input_buffer.insert(usize::from(self.current_column-self.path_len_u16()-1), ch);
                            //self.current_input_buffer.push(ch);
                            self.current_column+=1;
                            self.edit_input_buffer();
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
                                    self.edit_input_buffer()
                                }
                            }
                            
                       
                        }
                    },
                    KeyCode::Enter=>{
                        if e.kind == KeyEventKind::Press{
                            if self.current_input_buffer.is_empty(){
                                self.current_line += 1;
                                self.draw(app_data);
                                return AppAction::Nothing
                            }
                            let input_string = self.current_input_buffer.clone();
                            self.current_input_buffer.clear();
    
                            let components:Vec<&str> = input_string.split(" ").collect();
    
                            match components[0]{
                                "cd"=>{
                                    for i in 1..components.len(){
                                        if components[i] != " "{
                                            //try new path
                                            let mut temp_path:PathBuf = self.current_file_path.clone();
                                            temp_path.push(components[i]);
                                            if temp_path.exists() && temp_path.is_dir(){
                                                self.current_file_path.push(components[i]);
                                                self.current_file_path = self.normalize_path(self.current_file_path.to_str().unwrap());
                                                
                                                self.draw_output(String::from(" "));
                                                self.reset_cursor();
                                                self.current_line += 1;
                                                self.draw(app_data);
                                            }else{
                                               
                                                self.draw_output(String::from("folder ".to_owned()+temp_path.to_str().unwrap()+"  does not exist"));
                                                self.reset_cursor();
                                                self.current_line += 1;
                                                self.draw(app_data);
                                            }
                                            
                                            break;
                                        }
                                    }
                                },
                                "ls"=>{
                                    let paths = fs::read_dir(self.current_file_path.as_path()).unwrap();

                                    for path in paths {
                                        
                                        let full_path = path.unwrap().path();

                                    
                                        let relative_path = full_path.strip_prefix(&self.current_file_path)
                                            .unwrap_or_else(|_| &full_path)  
                                            .to_path_buf();
                                
                                        self.draw_output(format!("{}", relative_path.display()));
                                       
                                    }
                                    self.reset_cursor();
                                    self.current_line += 1;
                                    self.draw(app_data);
                                },
                                "cd.."=>{
                                    
                                    self.current_file_path.pop();
                                    
                                    self.draw_output(String::from(" "));
                                    self.reset_cursor();
                                    self.current_line += 1;
                                    self.draw(app_data);
                                
                                },
                                "load"=>{
                                    for i in 1..components.len(){
                                        if components[i] != " "{
                                            let mut target_file = self.current_file_path.clone();
                                            target_file.push(components[i]);
                                            if target_file.exists() && target_file.is_file(){
                                                self.draw_output(String::from(components[i])+" was loaded");
                                                self.reset_cursor();
                                                self.current_line += 1;
                                                self.draw(app_data);
                                                return AppAction::LoadPage(target_file);
                                            }else if target_file.is_dir() {
                                                self.draw_output(String::from(target_file.to_str().unwrap().to_owned()+" is not a file"));
                                            
                                            }else if target_file.exists() == false {
                                                self.draw_output(String::from(target_file.to_str().unwrap().to_owned()+" does not exist"));
                                          
                                            }
                                            self.reset_cursor();
                                            self.current_line += 1;
                                            self.draw(app_data);

                                            break;
                                        }
                                    }
                                }
                                _=>{
                                    //unknown command
                                    self.current_line += 1;
                                    self.draw_output(String::from("error command ".to_owned()+components[0] +" not recognized"));
                                    self.reset_cursor();
                                    self.current_line += 1;
                                    self.draw(app_data);
                                }
                            }
                        }
                 



                    },
                    _=>{}
                }
            },
            _=>{}
        }
       AppAction::Nothing
    }
}