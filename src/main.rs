use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use base64::encode;
use opencv::{imgcodecs, prelude::*, videoio};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/status", post(status))
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
    // this argument tells axum to parse the request body as JSON into a `CaptureRequest` type
    Json(payload): Json<CaptureRequest>,
) -> impl IntoResponse {
    tracing::debug!("Opening {}", payload.rtsp);
    let mut err_msg: String = "".to_string();
    let mut img_b64: String = "".to_string();
    let status_code: StatusCode;

    if let Ok(mut cam) = videoio::VideoCapture::from_file(&payload.rtsp, videoio::CAP_ANY) {
        let mut frame = Mat::default();
        let mut retry_count = 0;
        loop {
            if let Ok(true) = cam.read(&mut frame) {
                let params: opencv::core::Vector<i32> = opencv::core::Vector::new();
                let mut buf: opencv::core::Vector<u8> = opencv::core::Vector::new();
                if let Ok(true) = imgcodecs::imencode(".jpg", &frame, &mut buf, &params) {
                    img_b64 = encode(buf);
                    status_code = StatusCode::OK;
                    break;
                } else {
                    retry_count += 1;
                }
            } else {
                retry_count += 1;
            }
            if retry_count > 25 * 5 {
                err_msg = "Failed to open video stream.".to_string();
                status_code = StatusCode::REQUEST_TIMEOUT;
                break;
            }
        }
    } else {
        err_msg = "Failed to open video stream.".to_string();
        status_code = StatusCode::BAD_GATEWAY;
    }

    // this will be converted into a JSON response with a status code of `201 Created`
    let cap_result = CaptureResult { img_b64, err_msg };
    (status_code, Json(cap_result))
}

// the input to our `capture` handler
#[derive(Deserialize)]
struct CaptureRequest {
    rtsp: String,
}

// the output to our `capture` handler
#[derive(Serialize)]
struct CaptureResult {
    err_msg: String,
    img_b64: String,
}
