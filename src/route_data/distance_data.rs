use serde::{Deserialize, Serialize};

use crate::entities::point::Point;

#[derive(Serialize, Deserialize)]
pub struct DistanceRouteData {
    first_point: Point,
    second_point: Point,
}
