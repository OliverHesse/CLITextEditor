use std::{borrow::BorrowMut, io::{self, stdout, Write}, path::PathBuf};

use crossterm::{cursor::{DisableBlinking, Hide, MoveTo, Show}, event::{Event, KeyCode, KeyEventKind, KeyModifiers}, queue, style::{self, Color, Print, SetBackgroundColor}, terminal::{self, Clear, ClearType, DisableLineWrap}, ExecutableCommand, QueueableCommand};

use crate::{page_libs::{PageCore, PageData, PageError}, pages::{file_navigation_page::{self, FileNavigation}, text_file_page::TextFilePage}};


pub struct AppData{
    pub pages_data:Vec<PageData>,
    pub active_page:usize
}

pub struct App{

    pub pages:Vec<Box<dyn PageCore>>,
    pub active_page:usize,
    pub mode:AppMode,
    pub input_buffer:String,
    pub current_column:u16
}
#[derive(PartialEq)]
pub enum AppMode{
    Normal,
    Command
}
pub enum AppError{
    NoPages,
    ActivePageOutOfRange,
    PageDoesNotExist,
    PageError(PageError),
}
pub enum AppAction{
    ChangePage(usize),
    ClosePage(usize),
    LoadPage(PathBuf),
    Error(String),
    Nothing,

}
impl App{
    pub fn add_page(&mut self,new_page:Box<dyn PageCore>){
        //the page for closing and opening files+commands will be separate from standard pages
        //these pages essentially override behavior
        //i could also add a page and just change out page data as needed.

        self.pages.push(new_page);
   }
    pub fn get_app_data(&self)->AppData{
        let mut all_page_data = Vec::<PageData>::new();
        for page in &self.pages{
            let page_data = page.get_page_data();
            match page_data{
                Some(data)=>all_page_data.push(data),
                None=>continue,
            }
        }
        return AppData{
            pages_data:all_page_data,
            active_page:self.active_page.clone()
        };
    }
    pub  fn remove_page(&mut self,page_id:usize)->Result<(), AppError>{
    
        if self.active_page >= self.pages.len(){
            return Err::<(),AppError>(AppError::ActivePageOutOfRange);
        }   
        self.pages.remove(page_id);
        if page_id < self.active_page{
            self.active_page -=1;
        }
        Ok(())
        
   }

   pub  fn change_page(&mut self,new_page:usize)->Result<(), AppError>{

        if self.active_page >= self.pages.len(){
            return Err::<(),AppError>(AppError::ActivePageOutOfRange);
        }   
        self.active_page = new_page;
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        stdout.execute(terminal::Clear(terminal::ClearType::Purge));
        stdout.execute(DisableLineWrap);
        stdout.execute(SetBackgroundColor(Color::Black));
        Ok(())
   }
   pub fn initialize(&mut self){

        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        stdout.execute(terminal::Clear(terminal::ClearType::Purge));
        stdout.execute(DisableLineWrap);
        stdout.execute(SetBackgroundColor(Color::Black));
       

        let app_data = self.get_app_data();
        self.pages[0].initial_draw(app_data);
   }
    //draws newly loaded page to screen
    pub  fn load_new_page(&mut self)->Result<(),AppError>{
        let app_data = self.get_app_data();
        self.pages[self.active_page].initial_draw(app_data);
        Ok(())
   }
   pub fn find_page_from_name(&mut self,page_name:String)->Result<usize,AppError>{
        for i in 0..self.pages.len(){
            
            if self.pages[i].get_page_name() == page_name{
                return Ok(i)
            }
        }
        return Err(AppError::PageDoesNotExist);
   }
   pub  fn get_all_pages_ref(&self)->Option<&Vec<Box<dyn PageCore>>>{
        return Some(&self.pages);
   }
   //draws loaded page to screen
   pub  fn load_page(&self)->Result<(),AppError>{

        self.pages[self.active_page].draw(self.get_app_data());

        Ok(())
   }
   pub fn setup_line_colour(&mut self,colour:Color){
        
        let (cols, rows) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,rows-1));

        stdout.queue(SetBackgroundColor(colour));
        for _ in 0..cols {
            stdout.write_all(b" ").unwrap();
            
        }

        stdout.queue(MoveTo(0,rows-1));
        stdout.queue(Show);
        stdout.flush();
   }
   pub fn edit_input_buffer(&mut self){
        let (cols, rows) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,rows-1));
        stdout.queue(Clear(ClearType::CurrentLine));
        stdout.queue(Print(self.input_buffer.clone()));
        stdout.queue(MoveTo(self.current_column,rows));
        stdout.queue(Show);
        stdout.flush();
   }
   pub  fn run(&mut self,iter_event:Event)->Result<(),AppError>{
      
        if self.pages.is_empty() {
            return Err::<(),AppError>(AppError::NoPages);
        }
        let app_data = self.get_app_data();
        match iter_event{
            Event::Key(event)=>{
                match event.code{
                    KeyCode::Tab=>if event.modifiers==KeyModifiers::CONTROL{
                        //temporarily use to open command prompt
                        self.mode = AppMode::Command;
    
                        self.setup_line_colour(Color::DarkGrey);
                    },
                    KeyCode::Enter=>{
                        if self.mode == AppMode::Command{
                            //process_command
                            self.setup_line_colour(Color::Black);
                            self.mode = AppMode::Normal;
                            
                            
                            let components:Vec<&str> = self.input_buffer.split(" ").collect();

                            match components[0]{
                                "select"=>{
                                    match self.find_page_from_name("view_loaded".to_string()){
                                        Ok(i)=>{
                                            println!("NOT HERE");
                                            let _ =self.change_page(i);
                                            self.pages[self.active_page].initial_draw(app_data);   
                                            self.current_column = 0;
                                            self.input_buffer.clear();  
                                             
                                            return Ok(());                                 
                                        },
                                        Err(page_error)=>return Err(page_error),
                                    }
                                    
                                },
                                "nav"=>{
                                    match self.find_page_from_name("navigation".to_string()){
                                        Ok(i)=>{
                                            let _ =self.change_page(i);
                                            self.pages[self.active_page].initial_draw(app_data);  
                                            self.current_column = 0;
                                            self.input_buffer.clear();   
                                            return Ok(());                                 
                                        },
                                        Err(page_error)=>return Err(page_error),
                                    }
                                    
                                }
                                _=>{ 
                                        self.current_column = 0;
                                        self.input_buffer.clear();
                                        self.pages[self.active_page].draw(app_data);
                                    },
                            }
                            
                            
                            
                            
                            return Ok(());

                        }
                    },
                    KeyCode::Char(ch)=>{
                        if self.mode == AppMode::Command{
                            if event.kind == KeyEventKind::Press{
                            
                                self.input_buffer.insert(usize::from(self.current_column), ch);
                                //self.current_input_buffer.push(ch);
                                self.current_column+=1;
                                self.edit_input_buffer();
                            }
                        }
                    },
                    KeyCode::Left=>{
                        if self.mode == AppMode::Command{
                            if event.kind == KeyEventKind::Press && self.current_column >= 0{
                                self.current_column -= 1;
                                self.edit_input_buffer();
                            }
                        }
                    },
                    KeyCode::Right=>{
                        if self.mode == AppMode::Command{
                            if event.kind == KeyEventKind::Press && usize::from(self.current_column) <= self.input_buffer.len(){
                                self.current_column += 1;
                                self.edit_input_buffer();
                            }
                        }
                    },
                    KeyCode::Backspace=>{
                        if event.kind == KeyEventKind::Press{
                            
                            let index_to_remove =usize::from(self.current_column);
                            if index_to_remove >= 0 && self.input_buffer.len() != 0 {
                                self.input_buffer.remove(usize::from(self.current_column));
                                self.current_column -= 1;
                                self.edit_input_buffer();
                            }
                            
                       
                        }
                    },
                    _=>{}
                }
            },
            _=>{}
        }
        if self.mode == AppMode::Command{
            return Ok(());
        }
        //detect all my predetermined behavior then run
        match self.pages[self.active_page].run(iter_event,app_data){
            AppAction::ChangePage(page_num)=>{},
            AppAction::ClosePage(page_num)=>{},
            AppAction::LoadPage(page_path)=>{
                let mut file = page_path.file_name().unwrap().to_str().unwrap();
                
                let (file_name, file_extension) = file
                    .split_once(".")
                    .map(|(name, ext)| (name, ext))
                    .unwrap_or((file, ""));
                match file_extension{
                    "txt"=>{
                        
                        let mut txt_page = TextFilePage{
                            page_data: PageData{
                                page_name:file_name.to_string(),
                                is_static:true,
                                file_path:page_path.clone(),
                                file_extension:file_extension.to_owned(),
                                file_name:file_name.to_string(),
                                is_fixed:true,
                            }
                        };
                        self.add_page(Box::new(txt_page));
                    },
                    _=>{}
                }

            },
            _=>{}
        }
        
        Ok(())
   }
}