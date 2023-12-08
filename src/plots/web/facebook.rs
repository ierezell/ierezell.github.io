use std::collections::HashMap;

use plotly::common::Title;
use plotly::{Bar, Histogram, Layout, Plot};
pub fn get_message_count_plot(messages_count: &HashMap<String, i32>) -> Plot {
    let mut msg_plot = Plot::new();

    for name in messages_count.keys() {
        msg_plot.add_trace(
            Bar::new(
                ["Messages"].to_vec(),
                [messages_count[&name.clone()]].to_vec(),
            )
            .name(name),
        )
    }

    let msg_layout = Layout::new().title(Title::new("Messages per participants"));

    msg_plot.set_layout(msg_layout);
    return msg_plot;
}

pub fn get_reaction_count_plot(reactions_count: &HashMap<String, i32>) -> Plot {
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

pub fn get_date_plot(dates: &HashMap<String, Vec<u32>>) -> Plot {
    let mut date_plot = Plot::new();

    for (name, d) in dates.iter() {
        date_plot.add_trace(
            Histogram::new(d.to_vec())
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

pub fn get_response_time_plot(responses_time: &HashMap<String, Vec<i64>>) -> Plot {
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
