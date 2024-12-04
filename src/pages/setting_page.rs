use std::io::{self, Write};

use crossterm::{cursor::{Hide, MoveTo, Show}, event::Event, style::Print, terminal::{Clear, ClearType}, QueueableCommand};

use crate::{app::{App, AppAction, AppData}, page_libs::{PageCore, PageData}};

pub struct Settings{
    pub page_data:PageData,
}

impl PageCore for Settings{
    fn get_page_data(&self)->Option<PageData> {
        return Some(self.page_data.clone())
    }
    fn initial_draw(&mut self,app_data:AppData)->AppAction {
        let mut stdout = io::stdout();
        stdout.queue(Hide);
        stdout.queue(MoveTo(0,0));
        stdout.queue(Clear(ClearType::CurrentLine));
        stdout.queue(Print("Setting page under construction"));
        stdout.queue(Show);
        stdout.flush();
        AppAction::Nothing
    }
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction {
        AppAction::Nothing
    }
}