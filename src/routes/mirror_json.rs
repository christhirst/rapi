use axum::{
    extract::{Path, Query},
    headers::UserAgent,
    http::{header::ToStrError, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use super::SharedData;
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MJR {
    message: String,
    message_fs: String,
}

pub async fn mirror_json_body(Json(body): Json<MirrorJson>) -> Json<MJR> {
    Json(MJR {
        message: body.message,
        message_fs: "testing".to_owned(),
    })
}

pub async fn path_var_body(Path(id): Path<i32>) -> String {
    id.to_string()
}

pub async fn path_params(Query(query): Query<MirrorJson>) -> Json<MirrorJson> {
    Json(query)
}

pub async fn m_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
pub async fn m_header(header: HeaderMap) -> String {
    let message_val = header.get("x-message").unwrap();
    message_val.to_str().unwrap().to_owned()
}

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_middleware(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

pub async fn set_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_error: ToStrError| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extensions = request.extensions_mut();

    extensions.insert(HeaderMessage(message));
    Ok(next.run(request).await)
}

pub async fn always_errors() -> Result<(), StatusCode> {
    Err(StatusCode::OK)
}
pub async fn return_201() -> Response {
    (StatusCode::CREATED, ()).into_response()
}
pub async fn return_json() -> Json<MJR> {
    let data = MJR {
        message: "message".to_owned(),
        message_fs: "message".to_owned(),
    };

    Json(data)
}

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    title:  String,    
    marketing: bool,
}
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

//#[derive(Deserialize, Debug)]
/* pub struct RequestUser {
    username: Option<String>,
    password: String,
} */

/* pub async fn validate_json(Json(user): Json<RequestUser>) {
    dbg!(user);
}  */
//Json(body): Json<MirrorJson>
 pub async fn create_task(Extension(db): Extension<Surreal<Db>>,Json(per): Json<Person>,) {
   
   ()
    
} 
