use my_models::MyModel;

#[async_trait::async_trait]
pub trait RandomRepo {
    async fn save(&self, my_model: &MyModel) -> Result<(), String>;
}