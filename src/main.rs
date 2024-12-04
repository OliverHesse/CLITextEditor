use std::{io, path::PathBuf};

use app::App;
use crossterm::{event::read, terminal::{self, EnterAlternateScreen}};
use page_libs::{PageCore, PageData};
use pages::{file_navigation_page::FileNavigation, setting_page::Settings};


mod page_libs;
mod display_libs;
mod app;
mod pages;

fn main() {
    println!("Hello, world!");
    main_loop();
}
fn main_loop()->io::Result<()>{
    let mut root_app = App{
        pages : Vec::<Box<dyn PageCore>>::new(),
        active_page :0
    };
    let mut nav_page = FileNavigation{
        current_line: 0,
        current_column: 0,
        current_file_path: PathBuf::new(),
        current_input_buffer:String::new(),
        page_data: PageData{
            page_name:String::from("navigation"),
            is_static:false,
            file_path:String::new(),
            file_extension:String::new(),
            file_name:String::new()
        }
    };
    let mut set_page = Settings{
        page_data: PageData{
            page_name:String::from("settings"),
            is_static:true,
            file_path:String::new(),
            file_extension:String::new(),
            file_name:String::new()
        }
    };
    terminal::enable_raw_mode()?;
    let _screen = EnterAlternateScreen;
    root_app.add_page(Box::new(nav_page));
    root_app.add_page(Box::new(set_page));
    
    root_app.initialize();
    loop {
        
        root_app.run(read()?);
    }
}
