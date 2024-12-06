use std::{io, path::PathBuf};

use app::App;
use crossterm::{event::read, terminal::{self, EnterAlternateScreen}};
use page_libs::{PageCore, PageData};
use pages::{file_navigation_page::FileNavigation, setting_page::Settings, view_loaded_pages_page::ViewLoadedPages};


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
        active_page :0,
        mode:app::AppMode::Normal,
        input_buffer:String::new(),
        current_column:0
    };
    let mut nav_page = FileNavigation{
        current_line: 0,
        current_column: 0,
        current_file_path: PathBuf::new(),
        current_input_buffer:String::new(),
        page_data: PageData{
            page_name:String::from("navigation"),
            is_static:false,
            file_path:PathBuf::new(),
            file_extension:String::new(),
            file_name:String::new()
        }
    };
    let mut set_page = Settings{
        page_data: PageData{
            page_name:String::from("settings"),
            is_static:true,
            file_path:PathBuf::new(),
            file_extension:String::new(),
            file_name:String::new()
        }
    };
    let mut view_loaded = ViewLoadedPages{
        page_data: PageData{
            page_name:String::from("view_loaded"),
            is_static:true,
            file_path:PathBuf::new(),
            file_extension:String::new(),
            file_name:String::new()
        },
        current_line :0,
        text:Vec::new(),
    };
    terminal::enable_raw_mode()?;
    let _screen = EnterAlternateScreen;
    root_app.add_page(Box::new(nav_page));
    root_app.add_page(Box::new(set_page));
    root_app.add_page(Box::new(view_loaded));
    
    root_app.initialize();
    loop {
        
        let result = root_app.run(read()?);
        match result{
            Ok(())=>(),
            Err(data)=>println!("ther was an error")
        }
    }
}
