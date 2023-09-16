use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut nav = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = nav.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!("Error: {}", error);
                wait_for_key_press();
            } else {
                let user_input = get_user_input();
                match page.handle_input(user_input.trim()) {
                    Err(error) => {
                        println!("Error: {}", error);
                        wait_for_key_press();
                    },
                    Ok(action) => {
                        if let Some(action) = action {
                            if let Err(error) = nav.handle_action(action) {
                                println!("Error: {}", error);
                                wait_for_key_press();
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
