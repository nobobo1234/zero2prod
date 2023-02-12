use actix_web::{Responder, HttpResponse, web};
use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> impl Responder {
    // Make sure we use the result and error if something goes wrong
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        // Return 200 Ok if the query got executed well
        Ok(_) => HttpResponse::Ok().finish(),
        // Return 500 Internal Server Error if it goes wrong
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
