use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: String,
}

pub async fn index() -> Html<String> {
    let template = IndexTemplate {
        message: "Click me".into(),
    };
    Html(template.render().unwrap())
}