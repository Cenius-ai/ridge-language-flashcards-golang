use actix_web::{web, HttpResponse};
use askama::Template;

use crate::db;
use crate::models::{CardForm, Flashcard};
use crate::AppError;
use crate::AppState;

// ── Templates ────────────────────────────────────────────────────

#[derive(Template)]
#[template(path = "cards_list.html")]
struct CardsListTemplate {
    cards: Vec<Flashcard>,
    active_page: &'static str,
}

#[derive(Template)]
#[template(path = "card_new.html")]
struct CardNewTemplate {
    active_page: &'static str,
    errors: Vec<String>,
    front: String,
    back: String,
}

#[derive(Template)]
#[template(path = "card_edit.html")]
struct CardEditTemplate {
    card: Flashcard,
    active_page: &'static str,
    errors: Vec<String>,
    front: String,
    back: String,
}

// ── Handlers ─────────────────────────────────────────────────────

pub async fn list(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let conn = state.db.lock().unwrap();
    let cards = db::get_all_cards(&conn)?;
    let html = CardsListTemplate {
        cards,
        active_page: "cards",
    }
    .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn new_form() -> HttpResponse {
    let html = CardNewTemplate {
        active_page: "cards",
        errors: vec![],
        front: String::new(),
        back: String::new(),
    }
    .render()
    .unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn create(
    state: web::Data<AppState>,
    form: web::Form<CardForm>,
) -> Result<HttpResponse, AppError> {
    let mut errors: Vec<String> = vec![];

    let front = form.front.trim().to_string();
    let back = form.back.trim().to_string();

    if front.is_empty() {
        errors.push("Front text is required.".into());
    }
    if back.is_empty() {
        errors.push("Back text is required.".into());
    }

    if !errors.is_empty() {
        let html = CardNewTemplate {
            active_page: "cards",
            errors,
            front: form.front.clone(),
            back: form.back.clone(),
        }
        .render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(html));
    }

    let conn = state.db.lock().unwrap();
    db::create_card(&conn, &front, &back)?;

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/cards"))
        .finish())
}

pub async fn edit_form(
    state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = state.db.lock().unwrap();
    let card = db::get_card(&conn, id)?.ok_or(AppError::NotFound)?;

    let html = CardEditTemplate {
        front: card.front.clone(),
        back: card.back.clone(),
        card,
        active_page: "cards",
        errors: vec![],
    }
    .render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn update(
    state: web::Data<AppState>,
    path: web::Path<i64>,
    form: web::Form<CardForm>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let front = form.front.trim().to_string();
    let back = form.back.trim().to_string();
    let mut errors: Vec<String> = vec![];

    if front.is_empty() {
        errors.push("Front text is required.".into());
    }
    if back.is_empty() {
        errors.push("Back text is required.".into());
    }

    if !errors.is_empty() {
        let conn = state.db.lock().unwrap();
        let card = db::get_card(&conn, id)?.ok_or(AppError::NotFound)?;
        let html = CardEditTemplate {
            card,
            active_page: "cards",
            errors,
            front: form.front.clone(),
            back: form.back.clone(),
        }
        .render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(html));
    }

    let conn = state.db.lock().unwrap();
    db::update_card(&conn, id, &front, &back)?;

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/cards"))
        .finish())
}

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let conn = state.db.lock().unwrap();
    db::delete_card(&conn, id)?;

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/cards"))
        .finish())
}
