use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
    middleware::Next,
    http::{Request, StatusCode},
    BoxError,
};
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(limit)),
        }
    }

    pub async fn call<B>(
        self,
        req: Request<B>,
        next: Next<B>,
    ) -> Result<Response, StatusCode> {
        let permit = self.semaphore.acquire().await;
        if permit.is_err() {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        Ok(next.run(req).await)
    }
}

#[async_trait]
impl<B> FromRequest<B> for RateLimiter
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Implement rate limiter extraction
        Ok(RateLimiter::new(100)) // Example limit
    }
}

