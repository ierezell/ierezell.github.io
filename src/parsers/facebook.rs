use crate::parsers::base::BaseMessage;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Serialize, Deserialize, Clone)]
struct FacebookGif {
    uri: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FacebookShare {
    share_text: String,
    link: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FacebookReaction {
    reaction: String,
    actor: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FacebookPhoto {
    uri: String,
    creation_timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone)]
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

impl Into<BaseMessage> for FacebookMessage {
    fn into(self) -> BaseMessage {
        return BaseMessage {
            content: self.content,
            sender_name: self.sender_name,
            timestamp_ms: self.timestamp_ms,
        };
    }
}

pub fn parse_facebook(files: Vec<String>) -> (Vec<FacebookMessage>, HashSet<std::string::String>) {
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

#[cfg(test)]
mod tests {
    use super::parse_facebook;
    use std::fs::read_to_string;

    #[test]
    fn test_message_numbers() {
        let (messages, participants) = parse_facebook(vec![read_to_string(String::from(
            "./tests/assets/message_1.json",
        ))
        .unwrap()]);
        assert_eq!(messages.len(), 9519);
        assert_eq!(participants.len(), 2);
    }

    #[test]
    fn test_message_numbers_2() {
        let (messages, participants) = parse_facebook(vec![read_to_string(String::from(
            "./tests/assets/message_2.json",
        ))
        .unwrap()]);
        assert_eq!(messages.len(), 16);
        assert_eq!(participants.len(), 2);
    }
}
