use std::collections::HashMap;

pub enum Status {
  Closed,
  InProgress,
  Open,
  Resolved
}

pub struct Epic {
  description: String,
  name: String,
  status: Status,
  stories: Vec<i32>
}

impl Epic {
  pub fn new(name: String, description: String) -> Self {
    Epic {
      description,
      name,
      status: Status::Open,
      stories: vec![]
    }
  }
}

pub struct Story {
  description: String,
  name: String,
  status: Status
}

impl Story {
  pub fn new(name: String, description: String) -> Self {
    Story {
      description,
      name,
      status: Status::Open
    }
  }
}

pub struct DBState {
  last_item_id: i32,
  epics: HashMap<i32, Epic>,
  stories: HashMap<i32, Story>
}