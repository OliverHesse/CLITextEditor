use crossterm::event::Event;

use crate::{app::{App, AppAction, AppData},display_libs};
#[derive(Clone,Debug)]
pub struct PageData{
    pub page_name:String,
    pub is_static:bool,
    pub file_path:String,
    pub file_name:String,
    pub file_extension:String,

}

pub struct PageDisplayData{
    pub page_name:String,
    pub file_path:String
}

pub enum PageError{
    Generic(String)
}
pub enum PageResult{
    Generic()
}

pub trait PageCore{
    fn get_page_data(&self)->Option<PageData>{None}
    fn get_page_display_details(&self)->Option<PageDisplayData>{None}
    //logic to draw
    fn draw(&self,app_data:AppData){}
    //might want different behavior for initial draw
    fn initial_draw(&mut self,app_data:AppData)->AppAction{AppAction::Nothing}
    //run every iteration
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction{AppAction::Nothing}
    
    fn redraw_line(&self,line:usize){}
}