use actix_web::web;

use crate::controllers::todo_controller;

pub fn route_configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/ping", web::get().to(|| async { "pong!" }))
        .service(todo_controller::get_todos)
        .service(todo_controller::get_todo)
        .service(todo_controller::post_todo)
        .service(todo_controller::put_todo)
        .service(todo_controller::patch_todo)
        .service(todo_controller::delete_todo)
        .service(todo_controller::put_todo_finish)
        .service(todo_controller::delete_todo_finish);
}
