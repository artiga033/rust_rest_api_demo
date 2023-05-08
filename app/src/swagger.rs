use crate::controllers::todo_controller;
use crate::models::todo::{CreateTodoDto, TodoDto, TodoDtoList, UpdateTodoDto};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        todo_controller::get_todos,
        todo_controller::get_todo,
        todo_controller::post_todo,
        todo_controller::put_todo,
        todo_controller::patch_todo,
        todo_controller::delete_todo,
        todo_controller::put_todo_finish,
        todo_controller::delete_todo_finish,),
    components(
        schemas(CreateTodoDto,TodoDto,UpdateTodoDto,TodoDtoList)
    ),
    tags((name = "Todo", description = "Todo CRUD RESTful Api"))
)]
pub struct ApiDoc;
