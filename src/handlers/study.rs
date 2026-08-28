use actix_web::{web, HttpResponse};
use askama::Template;

use crate::db;
use crate::models::{AnswerForm, Flashcard};
use crate::AppError;
use crate::AppState;

#[derive(Template)]
#[template(path = "study.html")]
struct StudyTemplate<'a> {
    card: &'a Flashcard,
    due_count: i64,
    active_page: &'static str,
}

#[derive(Template)]
#[template(path = "study_complete.html")]
struct StudyCompleteTemplate {
    total: i64,
    active_page: &'static str,
}

/// Show the next due card, or a "done" message if none are due.
pub async fn study(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let conn = state.db.lock().unwrap();
    let card = db::get_due_card(&conn)?;
    let due_count = db::get_due_count(&conn)?;

    if let Some(ref c) = card {
        // Have to render and drop before conn unlocks — but the template needs c to live.
        // Work around by cloning card data.
        let card_owned = c.clone();
        drop(conn);
        let html = StudyTemplate {
            card: &card_owned,
            due_count,
            active_page: "study",
        }
        .render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        let total = db::get_total_count(&conn)?;
        let html = StudyCompleteTemplate {
            total,
            active_page: "study",
        }
        .render()?;
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    }
}

/// Process a rating answer, update SM-2, and redirect back to /study.
pub async fn answer(
    state: web::Data<AppState>,
    form: web::Form<AnswerForm>,
) -> Result<HttpResponse, AppError> {
    let rating: crate::sm2::Rating = form
        .rating
        .parse()
        .map_err(|_| AppError::BadRequest("invalid rating".into()))?;

    let conn = state.db.lock().unwrap();
    db::answer_card(&conn, form.card_id, rating)?;

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/study"))
        .finish())
}
