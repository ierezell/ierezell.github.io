use chrono::prelude::{NaiveDateTime, Timelike};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
struct FacebookGif {
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookShare {
    share_text: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookReaction {
    reaction: String,
    actor: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookPhoto {
    uri: String,
    creation_timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookMessage {
    sender_name: String,
    pub timestamp_ms: i64,
    content: Option<String>,
    photos: Option<Vec<FacebookPhoto>>,
    reactions: Option<Vec<FacebookReaction>>,
    shares: Option<FacebookShare>,
    gif: Option<Vec<FacebookGif>>,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookParticipant {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookMessenger {
    pub participants: Vec<FacebookParticipant>,
    pub messages: Vec<FacebookMessage>,
}

pub fn get_message_counts(messages: &Vec<FacebookMessage>) -> HashMap<String, i32> {
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
    messages: &Vec<FacebookMessage>,
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

pub fn get_reactions_counts(messages: &Vec<FacebookMessage>) -> HashMap<String, i32> {
    // HashMap<String, HashMap<String, i32>> (per person per reaction)
    let mut reaction_count = HashMap::new();

    for msg in messages {
        let sender = msg.sender_name.clone();

        if let Some(reactions) = &msg.reactions {
            for reaction in reactions {
                if reaction.actor != sender {
                    match reaction_count.get(&reaction.actor) {
                        Some(count) => {
                            reaction_count.insert(reaction.actor.to_string(), count + 1);
                        }
                        None => {
                            reaction_count.insert(reaction.actor.to_string(), 1);
                        }
                    }
                }
            }
        }
    }

    return reaction_count;
}

pub fn get_message_response_times(
    messages: &Vec<FacebookMessage>,
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

pub fn message_parser(files: Vec<String>) -> (Vec<FacebookMessage>, HashSet<std::string::String>) {
    let mut messages: Vec<FacebookMessage> = vec![];
    let mut participants: HashSet<String> = HashSet::new();

    for file_content in files.iter() {
        let fb: FacebookMessenger =
            serde_json::from_str(file_content).expect("Unable to create facebook object");
        messages.extend(fb.messages);

        for p in fb.participants {
            participants.insert(p.name);
        }
    }

    messages.sort_by(|a, b| a.timestamp_ms.cmp(&b.timestamp_ms));

    return (messages, participants);
}

#[cfg(test)]
mod tests {
    use super::message_parser;
    use std::fs::read_to_string;

    #[test]
    fn test_message_numbers() {
        let (messages, participants) = message_parser(vec![read_to_string(String::from(
            "./tests/assets/test_message_numbers.json",
        ))
        .unwrap()]);
        assert_eq!(messages.len(), 9519);
        assert_eq!(participants.len(), 2);
    }

    #[test]
    fn test_message_numbers_2() {
        let (messages, participants) = message_parser(vec![read_to_string(String::from(
            "./tests/assets/test_message_numbers_2.json",
        ))
        .unwrap()]);
        assert_eq!(messages.len(), 16);
        assert_eq!(participants.len(), 2);
    }
}
