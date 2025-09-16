use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use chrono::Utc;
use log::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    info!("Adding '{}' '{}' as a new subscriber", form.email, form.name);

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
        .await {
        Ok(_) => {
            info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
