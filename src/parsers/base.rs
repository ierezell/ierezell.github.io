use serde::{Deserialize, Serialize};

use chrono::prelude::{NaiveDateTime, Timelike};

use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseMessage {
    pub sender_name: String,
    pub timestamp_ms: i64,
    pub content: Option<String>,
}

pub fn get_message_counts(messages: &Vec<BaseMessage>) -> HashMap<String, i32> {
    let mut msg_count: HashMap<String, i32> = HashMap::new();
    for msg in messages {
        let sender = msg.sender_name.clone();
        match msg_count.get(&sender) {
            Some(count) => {
                msg_count.insert(sender.to_string(), count + 1);
            }
            None => {
                msg_count.insert(sender.to_string(), 1);
            }
        }
    }

    return msg_count;
}

pub fn get_send_hours(
    messages: &Vec<BaseMessage>,
    participants: &HashSet<String>,
) -> HashMap<String, Vec<i64>> {
    let mut message_hours: HashMap<String, Vec<i64>> = HashMap::new();
    for p in participants {
        message_hours.insert(p.to_string(), vec![]);
    }

    for msg in messages {
        let sender = msg.sender_name.clone();

        if let Some(hours_for_user) = message_hours.get_mut(&sender) {
            if let Some(datetime) = NaiveDateTime::from_timestamp_millis(msg.timestamp_ms) {
                hours_for_user.push(datetime.hour() as i64);
            }
        }
    }

    return message_hours;
}

pub fn get_message_response_times(
    messages: &Vec<BaseMessage>,
    participants: &HashSet<String>,
) -> HashMap<String, Vec<i64>> {
    let mut response_times: HashMap<String, Vec<i64>> = HashMap::new();

    for p in participants {
        response_times.insert(p.to_string(), vec![]);
    }

    let mut messages_iter = messages.iter().peekable();

    while let Some(msg) = messages_iter.next() {
        if let Some(next_msg) = messages_iter.peek() {
            if next_msg.sender_name != msg.sender_name {
                if let Some(sender_response_times) = response_times.get_mut(&next_msg.sender_name) {
                    let mut response_delta_in_seconds =
                        (next_msg.timestamp_ms - msg.timestamp_ms) / 1000;

                    if response_delta_in_seconds > 86400 {
                        response_delta_in_seconds = 86400
                    }

                    sender_response_times.push(response_delta_in_seconds);
                }
            }
        }
    }

    return response_times;
}
