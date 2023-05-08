use crate::models::paged_list::{PagedList, PagedListQueryParams};
use crate::models::todo::{self, CreateTodoDto, Todo, TodoDto, UpdateTodoDto};
use actix_web::http::header;
use actix_web::{delete, get, patch, post, put, Responder};
use actix_web::{web, HttpResponse};
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

/// Get Todo Items
#[utoipa::path(
    tag = "Todo",
    params(PagedListQueryParams),
    responses(
        (status=200,body=TodoDtoList)
    )
)]
#[get("/todo")]
pub async fn get_todos(
    pl_params: web::Query<PagedListQueryParams>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let s = Todo::find();
    match PagedList::<TodoDto>::from_paginator(s, db.get_ref(), &pl_params).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get Todo Item with Id
#[utoipa::path(tag = "Todo")]
#[get("todo/{id}")]
pub async fn get_todo(id: web::Path<i32>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let entity = Todo::find_by_id(*id).one(db.get_ref()).await;
    if let Ok(Some(entity)) = entity {
        HttpResponse::Ok().json(TodoDto::from(entity))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[utoipa::path(tag="Todo",request_body(content=CreateTodoDto,content_type = "application/json"))]
#[post("/todo")]
pub async fn post_todo(
    dto: web::Json<CreateTodoDto>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let entity: todo::ActiveModel = dto.0.into();
    match entity.insert(db.get_ref()).await {
        Ok(entity) => HttpResponse::Created()
            .insert_header((header::LOCATION, format!("{}", entity.id)))
            .json(TodoDto::from(entity)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(tag = "Todo")]
#[put("/todo/{id}")]
pub async fn put_todo(
    id: web::Path<i32>,
    dto: web::Json<UpdateTodoDto>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let entity = Todo::find_by_id(*id).one(db.get_ref()).await;
    if entity.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let entity = entity.unwrap();
    if entity.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let mut entity: todo::ActiveModel = entity.unwrap().into();
    let dto = dto.0;
    entity.content = Set(dto.content);
    entity.expire_at = Set(dto.expire_at.unwrap_or("".into()));
    match entity.update(db.get_ref()).await {
        Ok(entity) => HttpResponse::Created().json(TodoDto::from(entity)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(tag = "Todo",request_body(content=UpdateTodoDto,content_type="application/json"))]
#[patch("/todo/{id}")]
pub async fn patch_todo(
    id: web::Path<i32>,
    dto: web::Json<UpdateTodoDto>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let entity = Todo::find_by_id(*id).one(db.get_ref()).await;
    if entity.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let entity = entity.unwrap();
    if entity.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let mut entity: todo::ActiveModel = entity.unwrap().into();
    let dto = dto.0;
    if dto.content.is_some() {
        entity.content = Set(dto.content);
    }
    if let Some(exp_at) = dto.expire_at {
        entity.expire_at = Set(exp_at);
    }
    match entity.update(db.get_ref()).await {
        Ok(entity) => HttpResponse::Created().json(TodoDto::from(entity)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[utoipa::path(tag = "Todo")]
#[delete("/todo/{id}")]
pub async fn delete_todo(id: web::Path<i32>, db: web::Data<DatabaseConnection>) -> impl Responder {
    match Todo::delete_by_id(*id).exec(db.get_ref()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(tag = "Todo")]
#[put("/todo/{id}/finish")]
pub async fn put_todo_finish(
    id: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let entity = Todo::find_by_id(*id).one(db.get_ref()).await;
    if entity.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let entity = entity.unwrap();
    if entity.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let mut entity: todo::ActiveModel = entity.unwrap().into();
    entity.is_finished = Set(true);
    match entity.update(db.get_ref()).await {
        Ok(entity) => HttpResponse::Created().json(TodoDto::from(entity)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(tag = "Todo")]
#[delete("/todo/{id}/finish")]
pub async fn delete_todo_finish(
    id: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let entity = Todo::find_by_id(*id).one(db.get_ref()).await;
    if entity.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let entity = entity.unwrap();
    if entity.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let mut entity: todo::ActiveModel = entity.unwrap().into();
    entity.is_finished = Set(false);
    match entity.update(db.get_ref()).await {
        Ok(entity) => HttpResponse::Created().json(TodoDto::from(entity)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
