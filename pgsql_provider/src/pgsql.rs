use async_trait::async_trait;
use sqlx::PgPool;
use my_models::MyModel;
use repository_traits::my_traits::RandomRepo;
use chrono::Utc;

pub struct PgSqlRandomRepo {
    pool: PgPool,
}

impl PgSqlRandomRepo{
    pub async fn new(constr:&str)->Result<Self,String>{
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(constr)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self {pool})
    }
}
#[async_trait::async_trait]
impl RandomRepo for PgSqlRandomRepo {
    async fn save(&self, my_model: &MyModel) -> Result<(), String> {
        sqlx::query(r#"INSERT INTO test (mystring, myint, mydate) VALUES ($1, $2, $3)"#)
        .bind(&my_model.my_string)
        .bind(my_model.my_i32)
        .bind(my_model.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}