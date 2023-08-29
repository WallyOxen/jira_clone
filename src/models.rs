use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Status {
  Closed,
  InProgress,
  Open,
  Resolved
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Epic {
  pub description: String,
  pub name: String,
  pub status: Status,
  pub stories: Vec<u32>
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Story {
  pub description: String,
  pub name: String,
  pub status: Status
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DBState {
  pub last_item_id: u32,
  pub epics: HashMap<u32, Epic>,
  pub stories: HashMap<u32, Story>
}