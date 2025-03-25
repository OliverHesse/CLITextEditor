use std::path::PathBuf;

use crossterm::event::Event;

use crate::{app::{App, AppAction, AppData},display_libs};
#[derive(Clone,Debug)]
pub struct PageData{
    pub page_name:String,
    pub is_static:bool,
    pub file_path:PathBuf,
    pub file_name:String,
    pub file_extension:String,
    pub is_fixed:bool,
}

pub struct PageDisplayData{
    pub page_name:String,
    pub file_path:PathBuf
}

pub enum PageError{
    Generic(String)
}
pub enum PageResult{
    Generic()
}

impl PageData{
    pub fn get_page_display_details(&self)->PageDisplayData{
        return PageDisplayData{
            page_name:self.page_name.clone(),
            file_path:self.file_path.clone()
        };
    }
}

pub trait PageCore{
    fn get_page_name(&self)->String{String::new()}
    fn get_page_data(&self)->Option<PageData>{None}
    fn get_page_display_details(&self)->Option<PageDisplayData>{None}
    //logic to draw
    fn draw(&self,app_data:AppData)->AppAction{AppAction::Nothing}
    //might want different behavior for initial draw
    fn initial_draw(&mut self,app_data:AppData)->AppAction{AppAction::Nothing}
    //run every iteration
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction{AppAction::Nothing}
    
    fn redraw_line(&self,line:usize){}
}