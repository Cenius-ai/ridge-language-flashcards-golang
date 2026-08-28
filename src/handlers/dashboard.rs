use actix_web::{web, HttpResponse};
use askama::Template;

use crate::db;
use crate::AppError;
use crate::AppState;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    due_count: i64,
    total: i64,
    active_page: &'static str,
}

pub async fn dashboard(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let conn = state.db.lock().unwrap();
    let due_count = db::get_due_count(&conn)?;
    let total = db::get_total_count(&conn)?;

    let html = DashboardTemplate {
        due_count,
        total,
        active_page: "dashboard",
    }
    .render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
