
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Action1,
    Action2,
    Action3(i32)
}

pub type LogResult<T> = Result<T, String>;

pub trait LogResultUtil<T> {
    fn wrap_up(self, msg: &'static str) -> LogResult<T>;
}

impl<T,E:Error> LogResultUtil<T> for Result<T,E> {
    fn wrap_up(self, msg: &'static str) -> LogResult<T> {
        self.map_err(|x| format!("{}: {}", msg, x))
    }
}
