use axum::{
    extract::{Extension, Json, Path},
    handler::{delete, get, patch, post},
    routing::BoxRoute,
    Router,
};
use diesel::{prelude::*, PgConnection};
use dotenv::dotenv;
use hyper::{Server};
use std::net::SocketAddr;
use tower::ServiceBuilder;

mod models;
mod schema;

use crate::models::Task;
use crate::schema::tasks;

// Main function
#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Connect to the database
    let conn = PgConnection::establish(&database_url).expect("Error connecting to database");

    // Create a connection pool
    let conn_pool = std::sync::Arc::new(conn);

    // Set up the router
    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).patch(update_task).delete(delete_task))
        .layer(
            ServiceBuilder::new()
                .layer(crate::ConnectionLayer::new(conn_pool))
                .into_inner(),
        )
        .handle_error(|error: BoxError| {
            // Handle errors
            // ...
            unimplemented!()
        })
        .boxed();

    // Start the server
    let server = Server::bind(&addr).serve(app);
    println!("Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    
} // End of main function

// Define a custom layer to pass the database connection pool
pub struct ConnectionLayer {
    conn_pool: std::sync::Arc<PgConnection>,
}

impl ConnectionLayer {
    pub fn new(conn_pool: std::sync::Arc<PgConnection>) -> Self {
        Self { conn_pool }
    }
}

impl<S> tower::Layer<S> for ConnectionLayer {
    type Service = ConnectionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ConnectionService {
            inner,
            conn_pool: self.conn_pool.clone(),
        }
    }
}

pub struct ConnectionService<S> {
    inner: S,
    conn_pool: std::sync::Arc<PgConnection>,
}

impl<S, ReqBody> tower::Service<axum::http::Request<ReqBody>> for ConnectionService<S>
where
    S: tower::Service<axum::http::Request<ReqBody>, Response = axum::http::Response<axum::body::Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: axum::http::Request<ReqBody>) -> Self::Future {
        let conn = self.conn_pool.clone();
        axum::AddExtension::new(req, conn).map(|req| self.inner.call(req))
    }
}

async fn list_tasks(Extension(conn): Extension<PgConnection>) -> Result<Json<Vec<Task>>, BoxError> {
    // ...
    unimplemented!()
}

async fn create_task(Json(payload): Json<Task>, Extension(conn): Extension<PgConnection>) -> Result<Json<Task>, BoxError> {
    // ...
    unimplemented!()
}

async fn get_task(Path(id): Path<Uuid>, Extension(conn): Extension<PgConnection>) -> Result<Json<Task>, BoxError> {
    // ...
    unimplemented!()
}

async fn update_task(Path(id): Path<Uuid>, Json(payload): Json<Task>, Extension(conn): Extension<PgConnection>) -> Result<Json<Task>, BoxError> {
    // ...
    unimplemented!()
}

async fn delete_task(Path(id): Path<Uuid>, Extension(conn): Extension<PgConnection>) -> Result<Json<()>, BoxError> {
    // ...
    unimplemented!()
}