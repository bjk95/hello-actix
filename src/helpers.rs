use serde::Serialize;
use actix_web::web::Json;

pub fn respond_json<T>(data: T) -> Result<Json<T>, u8>
where
    T: Serialize,
{
    Ok(Json(data))
}
