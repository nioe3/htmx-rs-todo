use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use crate::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "todos.html")]
pub struct TodosTemplate{
    pub todos: Vec<Todo>,
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}