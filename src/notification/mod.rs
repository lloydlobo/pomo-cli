//
// Code copied and slightly modified from [24seconds/rust-cli-pomodoro](https://github.com/24seconds/rust-cli-pomodoro).
//

use chrono::{
    DateTime,
    Duration,
    Utc,
};

pub mod notify;

/// The notification scheme used to store `pompom` usage entries in the database.
#[derive(Debug)]
pub struct Notification {
    id: u16,
    description: String,
    work_time: u16,
    break_time: u16,
    created_at: DateTime<Utc>,
    work_expired_at: DateTime<Utc>,
    break_expired_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(id: u16, work_time: u16, break_time: u16, created_at: DateTime<Utc>) -> Self {
        let work_expired_at = created_at + Duration::minutes(work_time as i64);
        let break_expired_at = work_expired_at + Duration::minutes(break_time as i64);

        Self {
            id,
            description: String::from("sample description"),
            work_time,
            break_time,
            created_at,
            work_expired_at,
            break_expired_at,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_start_at(&self) -> DateTime<Utc> {
        let last_expired_at = self.work_expired_at.max(self.break_expired_at);
        let duration = Duration::minutes((self.work_time + self.break_time) as i64);
        last_expired_at - duration
    }
}
