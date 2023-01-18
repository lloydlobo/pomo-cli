use std::time::{
    Duration,
    SystemTime,
};

use humantime_serde::Serde;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
struct Foo {
    #[serde(with = "humantime_serde")]
    timeout: Duration,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    time: Option<SystemTime>,
}

#[derive(Serialize, Deserialize)]
struct FooSerdeWrapper {
    timeout: Vec<Serde<SystemTime>>,
}
