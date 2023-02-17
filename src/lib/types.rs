#[derive(Debug)]
pub struct Data {
    pub db: sqlx::PgPool,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
