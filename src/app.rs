use std::{borrow::BorrowMut, io};

use crossterm::{cursor::DisableBlinking, event::{Event, KeyCode, KeyModifiers}, queue, terminal::{self, DisableLineWrap}, ExecutableCommand, QueueableCommand};

use crate::{page_libs::{PageCore, PageData, PageError}, pages::file_navigation_page::{self, FileNavigation}};


pub struct AppData{
    pub pages_data:Vec<PageData>,
    pub active_page:usize
}

pub struct App{

    pub pages:Vec<Box<dyn PageCore>>,
    pub active_page:usize

}

pub enum AppError{
    NoPages,
    ActivePageOutOfRange,
    PageError(PageError),
}
pub enum AppAction{
    ChangePage(usize),
    ClosePage(usize),
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
                None=>println!("this page had no data"),
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
        Ok(())
   }
   pub fn initialize(&mut self){

        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        stdout.execute(terminal::Clear(terminal::ClearType::Purge));
        stdout.execute(DisableLineWrap);
       

        let app_data = self.get_app_data();
        self.pages[0].initial_draw(app_data);
   }
    //draws newly loaded page to screen
    pub  fn load_new_page(&mut self)->Result<(),AppError>{
        let app_data = self.get_app_data();
        self.pages[self.active_page].initial_draw(app_data);
        Ok(())
   }
   pub  fn get_all_pages_ref(&self)->Option<&Vec<Box<dyn PageCore>>>{
        return Some(&self.pages);
   }
   //draws loaded page to screen
   pub  fn load_page(&self)->Result<(),AppError>{

        self.pages[self.active_page].draw(self.get_app_data());

        Ok(())
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
                    },
                    _=>{}
                }
            },
            _=>{}
        }
        //detect all my predetermined behavior then run
        self.pages[self.active_page].run(iter_event,app_data);
        
        Ok(())
   }
}