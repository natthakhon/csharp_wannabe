use axum::{extract::State,response::IntoResponse,Json,http::StatusCode};
use repository_traits::RandomRepo;
use std::sync::Arc;
use crate::app_state::AppState;
use my_models::MyModel;

pub async fn test_controller(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<MyModel>) -> Result<impl IntoResponse, (StatusCode, String)>{

        state.repo.save(&payload).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(StatusCode::OK)
}