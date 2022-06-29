use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use opencv::{prelude::*, videoio, imgcodecs};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/status", get(status))
        // `POST /users` goes to `create_user`
        .route("/capture", post(capture));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn status() -> &'static str {
    "I'm OK!"
}

async fn capture(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CaptureRequest>,
) -> impl IntoResponse {
    println!("{}", payload.rtsp);
    if let Ok(mut cam) = videoio::VideoCapture::from_file(&payload.rtsp, videoio::CAP_ANY) {
        let mut frame = Mat::default();
        loop {
            if let Ok(result) = cam.read(&mut frame) {
                if result == true {
                    let params: opencv::core::Vector<i32> = opencv::core::Vector::new();
                    let _r = imgcodecs::imwrite("test.jpg", &frame, &params);
                    break;
                };
            };
        }
    };
    // this will be converted into a JSON response with a status code of `201 Created`
    let cap_result = CaptureResult {
        id: 0,
        img_b64: "SOME BASE 65 STRING".to_string(),
    };
    (StatusCode::CREATED, Json(cap_result))
}

// the input to our `capture` handler
#[derive(Deserialize)]
struct CaptureRequest {
    rtsp: String,
}

// the output to our `capture` handler
#[derive(Serialize)]
struct CaptureResult {
    id: u64,
    img_b64: String,
}
