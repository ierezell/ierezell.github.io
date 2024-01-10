use ratatui::style::{Color as RatatuiColor, Style as RatatuiStyle, Stylize};
use ratatui::widgets::{
    Bar as RatatuiBar, BarChart as RatatuiBarChart, BarGroup as RatatuiBarGroup,
    Block as RatatuiBlock, Borders as RatatuiBorders,
};
use std::collections::{BTreeMap, HashMap};

fn get_histogram(data: &HashMap<String, Vec<i64>>, num_buckets: i64) -> RatatuiBarChart<'static> {
    let mut all_values_per_participants: HashMap<String, BTreeMap<i64, i32>> =
        HashMap::from_iter(data.keys().map(|name| {
            let tree_map: BTreeMap<i64, i32> = BTreeMap::new();
            return (name.clone(), tree_map);
        }));

    let mut span_max = 0;
    let mut span_min = 0;
    for (name, times) in data.iter() {
        for time in times.iter() {
            let tree_map_for_name = all_values_per_participants.get_mut(name).expect("No name");
            let _ = *tree_map_for_name
                .entry(*time)
                .and_modify(|current| *current += 1)
                .or_insert(1);
            if time > &span_max {
                span_max = *time;
            }
            if time < &span_min {
                span_min = *time;
            }
        }
    }

    let span = span_max - span_min;

    let range = span + (span % num_buckets);

    let bucket_size = (range / num_buckets).max(1);

    let mut bar_chart: RatatuiBarChart<'static> = RatatuiBarChart::default()
        .block(RatatuiBlock::default().borders(RatatuiBorders::ALL))
        .label_style(RatatuiStyle::new().white());

    for idx in 0..num_buckets {
        let start = span_min + idx * bucket_size;
        let end = span_min + (idx + 1) * bucket_size;
        let mut bars = Vec::new();
        for (name_idx, name) in data.keys().enumerate() {
            let sum_range = all_values_per_participants[name]
                .range(start..end)
                .fold(0, |acc, x| acc + x.1);

            bars.push(
                RatatuiBar::default()
                    .value(sum_range as u64)
                    .label(format!("{}", name).into())
                    .style(RatatuiStyle::default().fg(match name_idx {
                        0 => RatatuiColor::Cyan,
                        1 => RatatuiColor::Yellow,
                        _ => RatatuiColor::Black,
                    })),
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
pub fn get_message_count_plot_cli(
    messages_count: &HashMap<String, i32>,
) -> RatatuiBarChart<'static> {
    let mut bar_chart: RatatuiBarChart<'static> = RatatuiBarChart::default()
        .block(
            RatatuiBlock::default()
                .title("Messages count")
                .borders(RatatuiBorders::ALL),
        )
        // .bar_width(30)
        // .bar_gap(2)
        // .group_gap(3)
        .label_style(RatatuiStyle::new().white());

    let mut bars = Vec::new();
    for (name_idx, name) in messages_count.keys().enumerate() {
        bars.push(
            RatatuiBar::default()
                .value(messages_count[name] as u64)
                .label(format!("{}", name).into())
                .style(RatatuiStyle::default().fg(match name_idx {
                    0 => RatatuiColor::Cyan,
                    1 => RatatuiColor::Yellow,
                    _ => RatatuiColor::Black,
                })),
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
        // .bar_width(6)
        // .bar_gap(2)
        // .group_gap(3)
        .label_style(RatatuiStyle::new().white());

    let mut bars = Vec::new();
    for (name_idx, name) in reaction_count.keys().enumerate() {
        bars.push(
            RatatuiBar::default()
                .value(reaction_count[name] as u64)
                .label(format!("{}", name).into())
                .style(RatatuiStyle::default().fg(match name_idx {
                    0 => RatatuiColor::Cyan,
                    1 => RatatuiColor::Yellow,
                    _ => RatatuiColor::Black,
                })),
        )
    }

    bar_chart = bar_chart.data(RatatuiBarGroup::default().bars(&bars));

    return bar_chart;
}

pub fn get_hour_plot_cli(hours: &HashMap<String, Vec<i64>>) -> RatatuiBarChart<'static> {
    return get_histogram(hours, 24).block(
        RatatuiBlock::default()
            .title("Hours")
            .borders(RatatuiBorders::ALL),
    );
}

pub fn get_response_time_plot_cli(
    responses_time: &HashMap<String, Vec<i64>>,
) -> RatatuiBarChart<'static> {
    fn percentile_of_sorted(sorted_samples: &[i64], pct: f64) -> f64 {
        assert!(!sorted_samples.is_empty());
        if sorted_samples.len() == 1 {
            return sorted_samples[0] as f64;
        }
        let zero: f64 = 0.0;
        assert!(zero <= pct);
        let hundred = 100_f64;
        assert!(pct <= hundred);
        if pct == hundred {
            return sorted_samples[sorted_samples.len() - 1] as f64;
        }
        let length = sorted_samples.len() - 1;
        let rank = (pct / hundred) * length as f64;
        let left_rank = rank.floor();
        let d = rank - left_rank;
        let n = left_rank as usize;
        let lo = sorted_samples[n] as f64;
        let hi = sorted_samples[n + 1] as f64;
        lo + (hi - lo) * d
    }

    let mut filtered_response_times = HashMap::new();
    for (name, times) in responses_time.iter() {
        let ninety_percentile = percentile_of_sorted(times, 90.0);
        let filtered_response_time: Vec<i64> = times
            .clone()
            .into_iter()
            .filter(|x| x < &&(ninety_percentile as i64))
            .collect();

        filtered_response_times.insert(name.clone(), filtered_response_time);
    }
    return get_histogram(&filtered_response_times, 15).block(
        RatatuiBlock::default()
            .title("Response time")
            .borders(RatatuiBorders::ALL),
    );
}
