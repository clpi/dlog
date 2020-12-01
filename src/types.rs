use serde::{Serialize, Deserialize, };


pub enum Unit {
    Boolean,
    Instant,
    Duration,
    Distance,

}

#[derive(Serialize, Deserialize)]
pub struct Fact {
    key: String,
    val: String,
    date: String,
    time: String,
}
