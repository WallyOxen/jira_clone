use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
  pub create_epic: Box<dyn Fn() -> Epic>,
  pub create_story: Box<dyn Fn() -> Story>,
  pub delete_epic: Box<dyn Fn() -> bool>,
  pub delete_story: Box<dyn Fn() -> bool>,
  pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
  pub fn new() -> Self {
    Self {
      create_epic: Box::new(create_epic_prompt),
      create_story: Box::new(create_story_prompt),
      delete_epic: Box::new(delete_epic_prompt),
      delete_story: Box::new(delete_story_prompt),
      update_status: Box::new(update_status_prompt)
    }
  }
}

fn create_epic_prompt() -> Epic {
  println!("----------------------------");
  
  println!("Epic Name:");
  let epic_name = get_user_input();

  println!("Epic Description:");
  let epic_description = get_user_input();

  Epic::new(epic_name.trim().to_owned(), epic_description.trim().to_owned())
}

fn create_story_prompt() -> Story {
  println!("----------------------------");
  
  println!("Story Name:");
  let story_name = get_user_input();

  println!("Story Description:");
  let story_description = get_user_input();

  Story::new(story_name.trim().to_owned(), story_description.trim().to_owned())
}

fn delete_epic_prompt() -> bool {
  println!("----------------------------");
  
  println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:");
  let user_input = get_user_input();

  user_input.trim().eq("Y")
}

fn delete_story_prompt() -> bool {
  println!("----------------------------");
  
  println!("Are you sure you want to delete this story? [Y/n]:");
  let user_input = get_user_input();

  user_input.trim().eq("Y")
}

fn update_status_prompt() -> Option<Status> {
  println!("----------------------------");
  
  println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
  let user_input = get_user_input();

  let status = user_input.trim().parse::<u8>();

  if let Ok(status) = status {
    return match status {
      1 => Some(Status::Open),
      2 => Some(Status::InProgress),
      3 => Some(Status::Resolved),
      4 => Some(Status::Closed),
      _ => None
    }
  }

  None
}