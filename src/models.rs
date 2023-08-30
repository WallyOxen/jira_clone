use std::{ collections::HashMap, fmt::Display };

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
  NavigateToEpicDetail { epic_id: u32 },
  NavigateToStoryDetail { epic_id: u32, story_id: u32 },
  NavigateToPreviousPage,
  CreateEpic,
  UpdateEpicStatus { epic_id: u32 },
  DeleteEpic { epic_id: u32 },
  CreateStory { epic_id: u32 },
  UpdateStoryStatus { story_id: u32 },
  DeleteStory { epic_id: u32, story_id: u32 },
  Exit,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Status {
  Closed,
  InProgress,
  Open,
  Resolved
}

impl Display for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Closed => write!(f, "CLOSED"),
      Self::InProgress => write!(f, "IN PROGRESS"),
      Self::Open => write!(f, "OPEN"),
      Self::Resolved => write!(f, "RESOLVED")
    }
  }
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