use chrono::prelude::NaiveDateTime;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::base::BaseMessage;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhatsappMessage {
    pub base_message: BaseMessage,
}

pub fn parse_whatsapp(texts: Vec<String>) -> (Vec<WhatsappMessage>, HashSet<std::string::String>) {
    let date_name_header_regex = Regex::new(
        r"^(?P<date>\d{2}\/\d{2}\/\d{4}\,\s\d{2}\:\d{2})\s-\s(?P<name>[\w\s]+)\:\s(?P<message>.+)$",
    )
    .unwrap();
    let mut messages = Vec::<WhatsappMessage>::new();
    let mut name = "";
    let mut timestamp = 0;
    let mut content = "".to_string();

    for txt in &texts {
        for msg_line in txt.split("\n") {
            if date_name_header_regex.is_match(msg_line) {
                if name != "" {
                    messages.push(WhatsappMessage {
                        base_message: BaseMessage {
                            sender_name: name.to_string(),
                            timestamp_ms: timestamp,
                            content: Some(content),
                        },
                    });
                }

                let captures = date_name_header_regex
                    .captures(msg_line)
                    .expect("No capture");

                let date = captures.name("date").expect("No date").as_str();

                timestamp = NaiveDateTime::parse_from_str(date, "%d/%m/%Y, %H:%M")
                    .expect("Failed to parse date")
                    .and_utc()
                    .timestamp_millis();

                name = captures.name("name").expect("No name").as_str();

                content = captures
                    .name("message")
                    .expect("No message")
                    .as_str()
                    .to_string();
            } else {
                let space = " ".to_string();
                let new_content = space + msg_line;
                content += new_content.as_str();
            }
        }

        if content != "" {
            messages.push(WhatsappMessage {
                base_message: BaseMessage {
                    sender_name: name.to_string(),
                    timestamp_ms: timestamp,
                    content: Some(content.clone()),
                },
            });
        }
    }

    messages.sort_by(|a, b| {
        a.base_message
            .timestamp_ms
            .cmp(&b.base_message.timestamp_ms)
    });

    let participants =
        HashSet::from_iter(messages.iter().map(|m| m.base_message.sender_name.clone()));
    return (messages, participants);
}
