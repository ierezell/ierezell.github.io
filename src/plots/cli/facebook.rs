use std::collections::{BTreeMap, HashMap};

use ratatui::style::{Style as RatatuiStyle, Stylize};

use ratatui::widgets::{
    Bar as RatatuiBar, BarChart as RatatuiBarChart, BarGroup as RatatuiBarGroup,
    Block as RatatuiBlock, Borders as RatatuiBorders,
};

pub fn get_message_count_plot_cli(
    messages_count: &HashMap<String, i32>,
) -> RatatuiBarChart<'static> {
    let mut bar_chart: RatatuiBarChart<'static> = RatatuiBarChart::default()
        .block(
            RatatuiBlock::default()
                .title("Messages count")
                .borders(RatatuiBorders::ALL),
        )
        .bar_width(6)
        .bar_gap(2)
        .group_gap(3)
        .label_style(RatatuiStyle::new().white());

    let mut bars = Vec::new();
    for name in messages_count.keys() {
        bars.push(
            RatatuiBar::default()
                .value(messages_count[name] as u64)
                .label(format!("{} ", name).into()),
        )
    }

    bar_chart = bar_chart.data(RatatuiBarGroup::default().bars(&bars));

    return bar_chart;
}

pub fn get_reaction_count_plot_cli(
    reaction_count: &HashMap<String, i32>,
) -> RatatuiBarChart<'static> {
    let mut bar_chart: RatatuiBarChart<'static> = RatatuiBarChart::default()
        .block(
            RatatuiBlock::default()
                .title("Reactions count")
                .borders(RatatuiBorders::ALL),
        )
        .bar_width(6)
        .bar_gap(2)
        .group_gap(3)
        .label_style(RatatuiStyle::new().white());

    let mut bars = Vec::new();
    for name in reaction_count.keys() {
        bars.push(
            RatatuiBar::default()
                .value(reaction_count[name] as u64)
                .label(format!("{} ", name).into()),
        )
    }

    bar_chart = bar_chart.data(RatatuiBarGroup::default().bars(&bars));

    return bar_chart;
}

pub fn get_hour_plot_cli(hours: &HashMap<String, Vec<u32>>) -> RatatuiBarChart<'static> {
    let mut all_values_per_participants: HashMap<String, BTreeMap<u32, i32>> =
        HashMap::from_iter(hours.keys().map(|name| {
            let tree_map: BTreeMap<u32, i32> = BTreeMap::new();
            return (name.clone(), tree_map);
        }));

    let mut span_max = 0;
    let mut span_min = 0;
    for (name, msg_hours) in hours.iter() {
        for h in msg_hours.iter() {
            let tree_map_for_name = all_values_per_participants.get_mut(name).expect("No name");
            *tree_map_for_name.entry(*h).or_insert(0) += 1;
            if h > &span_max {
                span_max = *h;
            }
            if h < &span_min {
                span_min = *h;
            }
        }
    }

    let span = span_max - span_min;

    let num_buckets = 12; // 12 hours per day

    let range = span + (span % num_buckets);

    let bucket_size = (range / num_buckets).max(1);

    let mut bar_chart: RatatuiBarChart<'static> = RatatuiBarChart::default()
        .block(
            RatatuiBlock::default()
                .title("Hours")
                .borders(RatatuiBorders::ALL),
        )
        .bar_width(6)
        .bar_gap(2)
        .group_gap(3)
        .label_style(RatatuiStyle::new().white());

    for idx in 0..num_buckets {
        let start = span_min + idx * bucket_size;
        let end = span_min + (idx + 1) * bucket_size;
        let mut bars = Vec::new();
        for name in hours.keys() {
            bars.push(
                RatatuiBar::default()
                    .value(all_values_per_participants[name].range(start..end).count() as u64)
                    .label(format!("{} ", name).into()),
            )
        }

        bar_chart = bar_chart.data(
            RatatuiBarGroup::default()
                .bars(&bars)
                .label(format!("{} - {}", start, end).into()),
        );
    }

    return bar_chart;
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
        .bar_width(6)
        .bar_gap(2)
        .group_gap(3)
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
                .label(format!("{} - {}", start, end).into()),
        );
    }

    return bar_chart;
}
