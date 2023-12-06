use std::collections::{BTreeMap, HashMap};

use plotly::common::Title;
use plotly::{Bar, Histogram, Layout, Plot};
use ratatui::style::{Style as RatatuiStyle, Stylize};
use ratatui::widgets::{
    Bar as RatatuiBar, BarChart as RatatuiBarChart, BarGroup as RatatuiBarGroup,
    Block as RatatuiBlock, Borders as RatatuiBorders,
};
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

pub fn get_response_time_plot_cli(
    responses_time: &HashMap<String, Vec<i64>>,
) -> RatatuiBarChart<'static> {
    let mut all_values_per_participants: HashMap<String, BTreeMap<i64, i32>> =
        HashMap::from_iter(responses_time.keys().map(|name| {
            let tree_map: BTreeMap<i64, i32> = BTreeMap::new();
            return (name.clone(), tree_map);
        }));

    let mut span_max = 0;
    let mut span_min = 0;
    for (name, times) in responses_time.iter() {
        for time in times.iter() {
            let tree_map_for_name = all_values_per_participants.get_mut(name).expect("No name");
            *tree_map_for_name.entry(*time).or_insert(0) += 1;
            if time > &span_max {
                span_max = *time;
            }
            if time < &span_min {
                span_min = *time;
            }
        }
    }

    let span = span_max - span_min;

    let num_buckets = 10;

    let range = span + (span % num_buckets);

    let bucket_size = (range / num_buckets).max(1);

    let mut bar_chart: RatatuiBarChart<'static> = RatatuiBarChart::default()
        .block(
            RatatuiBlock::default()
                .title("Response time")
                .borders(RatatuiBorders::ALL),
        )
        .bar_width(3)
        .bar_gap(1)
        .group_gap(3)
        .bar_style(RatatuiStyle::new().yellow().on_red())
        .value_style(RatatuiStyle::new().red().bold())
        .label_style(RatatuiStyle::new().white());

    for idx in 0..num_buckets {
        let start = span_min + idx * bucket_size;
        let end = span_min + (idx + 1) * bucket_size;
        let mut bars = Vec::new();
        for name in responses_time.keys() {
            bars.push(
                RatatuiBar::default()
                    .value(all_values_per_participants[name].range(start..end).count() as u64)
                    .label(format!("{} ", name).into()),
            )
        }

        bar_chart = bar_chart.data(
            RatatuiBarGroup::default()
                .bars(&bars)
                .label(format!("{}-{}", start, end).into()),
        );
    }

    return bar_chart;
}
