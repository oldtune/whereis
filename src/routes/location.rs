use actix_web::{post, web::Json, HttpResponse, Responder};

use crate::route_data::distance_data::DistanceRouteData;

#[post("/location/distance")]
pub async fn calculate_distance(distance_data: Json<DistanceRouteData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
