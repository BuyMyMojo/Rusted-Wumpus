#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: String
}

#[derive(Debug, sqlx::FromRow)]
pub struct QuoteRow {
    pub id: String,
    pub quote: String,
    pub author: String,
}