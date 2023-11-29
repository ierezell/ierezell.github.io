use std::collections::HashSet;

use crate::parsers::facebook::{
    get_message_counts, get_message_response_times, get_reactions_counts, get_send_dates,
    FacebookMessage,
};

use plotly::common::Title;
use plotly::{Bar, Histogram, Layout, Plot};

pub fn get_message_count_plot(messages: &Vec<FacebookMessage>) -> Plot {
    let msg_count = get_message_counts(&messages);
    let mut msg_plot = Plot::new();

    for name in msg_count.keys() {
        msg_plot.add_trace(
            Bar::new(["Messages"].to_vec(), [msg_count[&name.clone()]].to_vec()).name(name),
        )
    }

    let msg_layout = Layout::new().title(Title::new("Messages per participants"));

    msg_plot.set_layout(msg_layout);
    return msg_plot;
}

pub fn get_reaction_count_plot(messages: &Vec<FacebookMessage>) -> Plot {
    let reactions_count = get_reactions_counts(&messages);
    let mut reaction_plot = Plot::new();

    for name in reactions_count.keys() {
        reaction_plot.add_trace(
            Bar::new(
                ["Reactions"].to_vec(),
                [reactions_count[&name.clone()]].to_vec(),
            )
            .name(name),
        )
    }

    let reaction_layout = Layout::new().title(Title::new("Reactions per participants"));

    reaction_plot.set_layout(reaction_layout);

    return reaction_plot;
}

pub fn get_date_plot(messages: &Vec<FacebookMessage>, participants: &HashSet<String>) -> Plot {
    let messages_date = get_send_dates(&messages, &participants);
    let mut date_plot = Plot::new();

    for (name, dates) in messages_date.iter() {
        date_plot.add_trace(
            Histogram::new(dates.to_vec())
                .x_axis("Hour")
                .y_axis("Count")
                .name(name),
        );
    }
    let date_layout = Layout::new().title(Title::new(
        "Average number of messages per hour of the day.",
    ));

    date_plot.set_layout(date_layout);

    return date_plot;
}

pub fn get_response_time_plot(
    messages: &Vec<FacebookMessage>,
    participants: &HashSet<String>,
) -> Plot {
    let responses_time = get_message_response_times(&messages, &participants);
    let mut responses_time_plot = Plot::new();
    for (name, times) in responses_time.iter() {
        responses_time_plot.add_trace(
            Histogram::new(times.to_vec())
                .x_axis("Time")
                .y_axis("Count")
                .name(name),
        );
    }
    let time_layout = Layout::new().title(Title::new("Average response time of messages."));

    responses_time_plot.set_layout(time_layout);

    return responses_time_plot;
}
