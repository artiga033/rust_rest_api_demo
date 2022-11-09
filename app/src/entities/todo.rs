pub use super::entities::{prelude::*, todo::*};

use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateTodoDto {
    pub expire_at: String,
    pub content: Option<String>,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct UpdateTodoDto {
    pub expire_at: Option<String>,
    pub content: Option<String>,
}
#[derive(Debug, Serialize, PartialEq)]
pub struct TodoDto {
    pub id: i32,
    pub expire_at: String,
    pub content: Option<String>,
    pub is_finished: bool,
}

impl From<CreateTodoDto> for ActiveModel {
    fn from(dto: CreateTodoDto) -> Self {
        Self {
            expire_at: ActiveValue::Set(dto.expire_at),
            content: ActiveValue::Set(dto.content),
            is_finished: ActiveValue::Set(false),
            ..Default::default()
        }
    }
}

impl From<Model> for TodoDto {
    fn from(entity: Model) -> Self {
        Self {
            id: entity.id,
            expire_at: entity.expire_at,
            content: entity.content,
            is_finished: entity.is_finished,
        }
    }
}
