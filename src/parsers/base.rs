use chrono::{DateTime, Timelike};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseMessage {
    pub sender_name: String,
    pub timestamp_ms: i64,
    pub content: Option<String>,
}

// cSpell: disable
const STOP_WORDS: &[&str] = &[
    "a",
    "à",
    "ã",
    "ã§a",
    "â¤",
    "ãa",
    "ãªtre",
    "about",
    "above",
    "after",
    "again",
    "against",
    "ai",
    "aie",
    "aient",
    "aies",
    "ait",
    "all",
    "aller",
    "alors",
    "am",
    "an",
    "and",
    "any",
    "are",
    "as",
    "at",
    "au",
    "aura",
    "aurai",
    "auraient",
    "aurais",
    "aurait",
    "auras",
    "aurez",
    "auriez",
    "aurions",
    "aurons",
    "auront",
    "aussi",
    "aux",
    "avaient",
    "avais",
    "avait",
    "avec",
    "avez",
    "aviez",
    "avions",
    "avoir",
    "avons",
    "ayant",
    "ayez",
    "ayons",
    "be",
    "because",
    "been",
    "before",
    "being",
    "below",
    "between",
    "bien",
    "bon",
    "both",
    "but",
    "by",
    "c'est",
    "c",
    "ca",
    "can",
    "ce",
    "ceci",
    "cela",
    "ces",
    "cet",
    "cette",
    "d",
    "ð",
    "ð",
    "ð\u{9f}\u{98}",
    "ð\u{9f}\u{98}\u{82}",
    "dans",
    "de",
    "des",
    "did",
    "do",
    "does",
    "doing",
    "don",
    "down",
    "du",
    "during",
    "each",
    "elle",
    "en",
    "es",
    "est",
    "et",
    "étaient",
    "étais",
    "était",
    "ete",
    "été",
    "êtes",
    "étiez",
    "étions",
    "etre",
    "être",
    "eu",
    "eue",
    "eues",
    "eûmes",
    "eurent",
    "eus",
    "eusse",
    "eussent",
    "eusses",
    "eussiez",
    "eussions",
    "eut",
    "eût",
    "eûtes",
    "eux",
    "faire",
    "fais",
    "fait",
    "few",
    "for",
    "from",
    "fûmes",
    "furent",
    "further",
    "fus",
    "fusse",
    "fussent",
    "fusses",
    "fussiez",
    "fussions",
    "fut",
    "fût",
    "fûtes",
    "ha",
    "had",
    "haha",
    "has",
    "have",
    "having",
    "he",
    "her",
    "here",
    "hers",
    "herself",
    "him",
    "himself",
    "his",
    "how",
    "i",
    "ici",
    "if",
    "il",
    "ils",
    "in",
    "into",
    "is",
    "it",
    "its",
    "itself",
    "j'ai",
    "j",
    "je",
    "just",
    "l",
    "la",
    "le",
    "les",
    "leur",
    "leurs",
    "lui",
    "m",
    "ma",
    "mãªme",
    "mÃªme",
    "mais",
    "me",
    "meme",
    "même",
    "mes",
    "message",
    "moi",
    "mon",
    "more",
    "most",
    "my",
    "myself",
    "n",
    "ne",
    "no",
    "nor",
    "nos",
    "not",
    "notre",
    "nous",
    "now",
    "of",
    "off",
    "oh",
    "Oh",
    "on",
    "once",
    "only",
    "ont",
    "or",
    "other",
    "ou",
    "où",
    "oui",
    "Oui",
    "our",
    "ours",
    "ourselves",
    "out",
    "over",
    "own",
    "par",
    "pas",
    "peu",
    "peut",
    "peux",
    "plus",
    "pour",
    "qu",
    "quand",
    "que",
    "quel",
    "quelle",
    "quelles",
    "quels",
    "qui",
    "rã©agi",
    "rÃ©agi",
    "s",
    "sa",
    "sais",
    "same",
    "sans",
    "se",
    "sera",
    "serai",
    "seraient",
    "serais",
    "serait",
    "seras",
    "serez",
    "seriez",
    "serions",
    "serons",
    "seront",
    "ses",
    "she",
    "should",
    "si",
    "so",
    "soi",
    "soient",
    "sois",
    "soit",
    "some",
    "sommes",
    "son",
    "sont",
    "soyez",
    "soyons",
    "such",
    "suis",
    "sur",
    "t",
    "ta",
    "te",
    "tes",
    "than",
    "that",
    "the",
    "their",
    "theirs",
    "them",
    "themselves",
    "then",
    "there",
    "these",
    "they",
    "this",
    "those",
    "through",
    "to",
    "toi",
    "ton",
    "too",
    "tout",
    "trop",
    "tu",
    "un",
    "under",
    "une",
    "until",
    "up",
    "va",
    "vais",
    "very",
    "veux",
    "vos",
    "votre",
    "vous",
    "was",
    "we",
    "were",
    "what",
    "when",
    "where",
    "which",
    "while",
    "who",
    "whom",
    "why",
    "will",
    "with",
    "y",
    "you",
    "your",
    "ð",
    "ð",
    "ð",
    "yours",
    "yourself",
    "yourselves",
];
// cSpell: enable

pub fn get_message_counts(messages: &Vec<BaseMessage>) -> HashMap<String, i32> {
    let mut msg_count: HashMap<String, i32> = HashMap::new();
    for msg in messages {
        msg_count
            .entry(msg.sender_name.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    return msg_count;
}

/// Get the number of messages in a row sent by each participant
/// p1, p1, p1, p2, p2, p1, p1, p2, p2, p2 => {p1: [3, 2], p2: [2,3]}
pub fn get_messages_num(messages: &Vec<BaseMessage>) -> HashMap<String, Vec<i64>> {
    let mut msg_count: HashMap<String, Vec<i64>> = HashMap::new();
    let mut current_sender_num_msg = 0;
    let mut current_sender = "".to_string();

    for msg in messages {
        let msg_sender = msg.sender_name.clone();
        if msg.sender_name != current_sender {
            if current_sender == "" {
                current_sender = msg_sender;
                current_sender_num_msg += 1;
            } else {
                msg_count
                    .entry(current_sender)
                    .and_modify(|v| v.push(current_sender_num_msg))
                    .or_insert_with(|| vec![current_sender_num_msg]);
                current_sender_num_msg = 0;
                current_sender = msg_sender;
            }
        } else {
            current_sender_num_msg += 1;
        }
    }

    if current_sender_num_msg > 0 {
        msg_count
            .entry(current_sender)
            .and_modify(|v| v.push(current_sender_num_msg));
    }

    return msg_count;
}

pub fn get_frequent_words(
    messages: &Vec<BaseMessage>,
    num_words: usize,
) -> HashMap<String, Vec<String>> {
    let mut word_counts: HashMap<String, HashMap<String, i64>> = HashMap::new();

    for msg in messages {
        if let Some(content) = &msg.content {
            for word in content.split_whitespace() {
                if STOP_WORDS.contains(&word.to_lowercase().as_str()) {
                    continue;
                }

                word_counts
                    .entry(msg.sender_name.clone())
                    .or_insert(HashMap::new())
                    .entry(word.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let mut msg_count: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in word_counts.iter() {
        let mut top_strings: Vec<(&String, &i64)> = v.iter().collect();
        top_strings.sort_by(|a, b| b.1.cmp(a.1));
        msg_count.insert(
            k.to_string(),
            top_strings
                .iter()
                .take(num_words)
                .map(|x| x.0.to_string())
                .collect(),
        );
    }

    return msg_count;
}

// pub fn get_emojis(messages: &Vec<BaseMessage>) -> HashMap<String, Vec<String>> {
//     let mut msg_count: HashMap<String, i32> = HashMap::new();
//     for msg in messages {
//         let sender = msg.sender_name.clone();
//         match msg_count.get(&sender) {
//             Some(count) => {
//                 msg_count.insert(sender.to_string(), count + 1);
//             }
//             None => {
//                 msg_count.insert(sender.to_string(), 1);
//             }
//         }
//     }

//     return msg_count;
// }

pub fn get_messages_length(messages: &Vec<BaseMessage>) -> HashMap<String, Vec<i64>> {
    let mut msg_count: HashMap<String, Vec<i64>> = HashMap::new();
    for msg in messages {
        let sender = msg.sender_name.clone();
        if let Some(ct) = &msg.content {
            msg_count
                .entry(sender)
                .and_modify(|e| e.push(ct.len() as i64))
                .or_insert(vec![ct.len() as i64]);
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
            if let Some(datetime) = DateTime::from_timestamp_millis(msg.timestamp_ms) {
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
