use std::io::{self, Write};
use std::env;
use std::fs;
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal;
use crossterm::{cursor::{Hide, MoveTo, Show}, event::Event, style::Print, terminal::{Clear, ClearType}, QueueableCommand};

use crate::{app::{App, AppAction, AppData}, page_libs::{PageCore, PageData}};
//TODO add change stack with CTRL Z and CTRL Y
//TODO this will prob include custom scrolling code cus crossterm behaves odd
pub struct TextFilePage{
    pub page_data:PageData,
    pub text:Vec<Vec<char>>,
    pub current_line:usize,
    pub current_column:u16,
    pub line_offset:usize
}

impl TextFilePage{
    fn redraw_screen(&self)->AppAction{
        //draw current_buffer
        //start = line_offset
        //end = line_offset+rows
        let (cols, rows) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        stdout.queue(MoveTo(0,0));
        
        stdout.queue(Hide);

        
        let mut iter_len = rows;
        if self.text.len() < usize::from(rows){
            iter_len = self.text.len().try_into().ok().unwrap();
        }
        for row_i in 0..iter_len{
            stdout.queue(MoveTo(0,row_i));
            stdout.queue(Clear(ClearType::CurrentLine));
            stdout.queue(Print(self.text[self.line_offset+usize::from(row_i)].iter().collect::<String>()));
        }

        let viewport_row = match u16::try_from(self.current_line-self.line_offset){
            Ok(v) => v,
            Err(e) => return AppAction::Error(format!("{:?}",e)),
        };
        stdout.queue(MoveTo(self.current_column,viewport_row));
        stdout.queue(Show);
        stdout.flush();
        return AppAction::Nothing;
        
    }

}


impl PageCore for TextFilePage{
    fn get_page_data(&self)->Option<PageData> {
        return Some(self.page_data.clone())
    }

    fn draw(&self,app_data:AppData)->AppAction{
        let mut stdout = io::stdout();

        stdout.queue(Hide);
        
        let viewport_row = match u16::try_from(self.current_line-self.line_offset){
            Ok(v) => v,
            Err(e) => return AppAction::Error(format!("{:?}",e)),
        };
        stdout.queue(MoveTo(0,viewport_row));
        stdout.queue(Print(self.text[self.current_line].iter().collect::<String>()));
        stdout.queue(MoveTo(self.current_column,viewport_row));
        stdout.queue(Show);
        stdout.flush();
        return AppAction::Nothing;
    }

    

    fn initial_draw(&mut self,app_data:AppData)->AppAction {
        let mut stdout = io::stdout();
        let (cols, rows) = terminal::size().unwrap();
        //so i dont reset anything if initial_draw is called again
        
        if self.text.is_empty() == false{

            self.draw(app_data);
            return AppAction::Nothing;
        }
        let contents = fs::read_to_string(self.page_data.file_path.as_path())
        .expect("Should have been able to read the file");
        let file_rows:Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
        let mut iter = file_rows.iter();
        
        stdout.queue(Hide);
        stdout.queue(SetBackgroundColor(Color::Black));
        for (i,row) in iter.enumerate(){
            let row_vec:Vec<char> = row.chars().collect();
            //TODO not super safe see what i can do about it
            if i<(rows as usize){
                stdout.queue(MoveTo(0,i.try_into().unwrap()));
                stdout.queue(Clear(ClearType::CurrentLine));
                stdout.queue(Print(row_vec.iter().collect::<String>()));
            }
            self.text.push(row_vec);

        }
        
        
        stdout.queue(MoveTo(0,0));

        stdout.queue(Show);
        stdout.flush();
        AppAction::Nothing
    }
    fn run(&mut self,iter_event:Event,app_data:AppData)->AppAction {

        match iter_event{
            Event::Key(e)=>{
                match e.code{            
                    KeyCode::Down=>{
                        if e.kind == KeyEventKind::Press && self.current_line < self.text.len()-1{
                            //valid change
                            //TODO do some extra math for end of line snapping
                            let rows = usize::from(terminal::size().unwrap().1);
                            
                            if usize::from(self.current_column+1) == self.text[self.current_line].len() || usize::from(self.current_column+1) >= self.text[self.current_line+1].len()-1{
                                //snap cursor
                                self.current_column = u16::try_from(self.text[self.current_line+1].len()).ok().unwrap()-1;
                            }      

                            //if curr_column is greater than next line snap aswell                      
                            self.current_line += 1;
                            
                            if rows+self.line_offset <= self.current_line{
                                self.line_offset += 1;
                                return self.redraw_screen();
                            }else{
                                return self.draw(app_data);
                            }
                            
                            
                        }

                    },
                    KeyCode::Up=>{
                        if e.kind == KeyEventKind::Press && self.current_line > 0{
                            //valid change
                            //TODO do some extra math for end of line snapping
                            let rows = usize::from(terminal::size().unwrap().1);
                            if usize::from(self.current_column+1) == self.text[self.current_line].len()|| usize::from(self.current_column+1) >= self.text[self.current_line-1].len()-1{
                                //snap cursor
                                self.current_column = u16::try_from(self.text[self.current_line-1].len()).ok().unwrap()-1
                            }                            
                            self.current_line -= 1;
                            if self.line_offset > self.current_line{
                                self.line_offset-=1;
                                return self.redraw_screen();
                            }else{
                                return self.draw(app_data);
                            }
                            
                            
                        }
                    },
                    KeyCode::Backspace=>{},
                    KeyCode::Left=>{
                        if e.kind == KeyEventKind::Press && usize::from(self.current_column) > 0{
                            self.current_column -= 1;
                            self.draw(app_data);
                        }
                    },
                    KeyCode::Right=>{
                        if e.kind == KeyEventKind::Press && usize::from(self.current_column) < self.text[self.current_line].len()-1{
                            self.current_column += 1;
                            self.draw(app_data);
                        }
                    },
                    KeyCode::Char(ch)=>{
                        if e.kind == KeyEventKind::Press{
                            if e.modifiers ==KeyModifiers::CONTROL{
                                match ch{
                                    's'=>{},
                                    'c'=>{},
                                    'v'=>{},
                                    'z'=>{},
                                    'y'=>{}
                                    _=>{}
                                }
                                return AppAction::Nothing;
                            }
 
                            self.text[self.current_line].insert(usize::from(self.current_column), ch);
                            self.current_column+=1;
                            self.draw(app_data);


                        }
                    },
                    _=>{}
                }
            }
            _=>{}
        }

        AppAction::Nothing
    }
    fn get_page_name(&self)->String{
        return self.page_data.file_name.clone();
    }
}