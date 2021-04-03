
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Action1,
    Action2,
    Action3(i32)
}
