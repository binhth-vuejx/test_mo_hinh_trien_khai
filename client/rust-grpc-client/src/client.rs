use axum::http::{
	header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}
};
use axum::{Router, routing::get, extract::State, http::StatusCode, response::IntoResponse, Json};

use tower_http::cors::CorsLayer;

use hello_world::greeter_client::GreeterClient;
use hello_world::Empty;

pub mod hello_world {
	tonic::include_proto!("helloworld");
}
use tonic::transport::Channel;

#[derive(Clone)]
struct AppState {
	client: GreeterClient<Channel>
}

async fn body1_kb(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	let response = state.client.body1_kb(request).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body10_kb(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	let response = state.client.body10_kb(request).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body100_kb(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	let response = state.client.body100_kb(request).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body1_mb(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	let response = state.client.body1_mb(request).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body5_mb(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	let response = state.client.body5_mb(request).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body10_mb(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	let response = state.client.body10_mb(request).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}


async fn body1_kb_bad_2(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body1_kb(request).await.unwrap().into_inner();
	let response = state.client.body1_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body10_kb_bad_2(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body10_kb(request).await.unwrap().into_inner();
	let response = state.client.body10_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body100_kb_bad_2(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body100_kb(request).await.unwrap().into_inner();
	let response = state.client.body100_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body1_mb_bad_2(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body1_mb(request).await.unwrap().into_inner();
	let response = state.client.body1_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body5_mb_bad_2(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body5_mb(request).await.unwrap().into_inner();
	let response = state.client.body5_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body10_mb_bad_2(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body10_mb(request).await.unwrap().into_inner();
	let response = state.client.body10_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}


async fn body1_kb_bad_3(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body1_kb(request).await.unwrap().into_inner();
	state.client.body1_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let response = state.client.body1_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body10_kb_bad_3(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body10_kb(request).await.unwrap().into_inner();
	state.client.body10_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let response = state.client.body10_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body100_kb_bad_3(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body100_kb(request).await.unwrap().into_inner();
	state.client.body100_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let response = state.client.body100_kb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body1_mb_bad_3(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body1_mb(request).await.unwrap().into_inner();
	state.client.body1_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let response = state.client.body1_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body5_mb_bad_3(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body5_mb(request).await.unwrap().into_inner();
	state.client.body5_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let response = state.client.body5_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}
async fn body10_mb_bad_3(State(mut state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let request = tonic::Request::new(Empty {});
	state.client.body10_mb(request).await.unwrap().into_inner();
	state.client.body10_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let response = state.client.body10_mb(tonic::Request::new(Empty {})).await.unwrap().into_inner();
	let json_response = serde_json::json!({
		"message": response.message
	});
	return Ok((StatusCode::OK, Json(json_response)));
}

#[tokio::main]
async fn main() {
let cors = CorsLayer::new()
	.allow_credentials(true)
	.allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
	
	let client = GreeterClient::connect("http://192.168.68.105:50051").await.unwrap();
	
	let state = AppState {
		client: client
	};
	let app = Router::new()
	.route("/body1_kb", get(body1_kb))
	.route("/body10_kb", get(body10_kb))
	.route("/body100_kb", get(body100_kb))
	.route("/body1_mb", get(body1_mb))
	.route("/body5_mb", get(body5_mb))
	.route("/body10_mb", get(body10_mb))
	.route("/body1_kb_bad_2", get(body1_kb_bad_2))
	.route("/body10_kb_bad_2", get(body10_kb_bad_2))
	.route("/body100_kb_bad_2", get(body100_kb_bad_2))
	.route("/body1_mb_bad_2", get(body1_mb_bad_2))
	.route("/body5_mb_bad_2", get(body5_mb_bad_2))
	.route("/body10_mb_bad_2", get(body10_mb_bad_2))
	.route("/body1_kb_bad_3", get(body1_kb_bad_3))
	.route("/body10_kb_bad_3", get(body10_kb_bad_3))
	.route("/body100_kb_bad_3", get(body100_kb_bad_3))
	.route("/body1_mb_bad_3", get(body1_mb_bad_3))
	.route("/body5_mb_bad_3", get(body5_mb_bad_3))
	.route("/body10_mb_bad_3", get(body10_mb_bad_3)).with_state(state).layer(cors);
	
	println!("🚀 Server started successfully");
	let listener = tokio::net::TcpListener::bind("0.0.0.0:50151").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
