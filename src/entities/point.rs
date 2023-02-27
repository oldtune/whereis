use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x_coord: f64,
    pub y_coord: f64,
}
