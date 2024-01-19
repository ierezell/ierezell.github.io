use clap::Parser;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use msg::{
    parsers::base::{
        get_frequent_words, get_message_counts, get_message_response_times, get_send_hours,
    },
    plots::cli::get_word_plot_cli,
};
use msg::{
    parsers::base::{get_messages_length, get_messages_num},
    plots::cli::{
        get_hour_plot_cli, get_message_count_plot_cli, get_message_num_plot_cli,
        get_reaction_count_plot_cli, get_response_time_plot_cli,
    },
};
use msg::{
    parsers::{facebook, file},
    plots::cli::get_message_length_plot_cli,
};
use ratatui::prelude::{Constraint, CrosstermBackend, Direction, Layout, Style, Terminal};
use ratatui::symbols;
use ratatui::widgets::{Block, Borders, Tabs};
use std::fs::read_to_string;
use std::io::stdout;

// Cli to parse facebook or whatsapp messages from local files.
#[derive(Parser, Debug)]
pub struct CliArgs {
    // Name of the person to extract messages from
    #[arg(short, long)]
    name: String,

    // Files to parse
    #[arg(short, long)]
    files: String,

    // Kind of file (whatsapp or facebook)
    #[arg(short, long)]
    kind: String,

    #[arg(short, long)]
    output: String,
}

pub fn main() {
    let args = CliArgs::parse();
    match args.kind.as_str() {
        "facebook" => {
            let correct_paths = file::facebook_file_parser(&args.files, &args.name);
            let mut files_data = Vec::new();
            for file in correct_paths.iter() {
                println!("Found {:?} file", file);
                files_data.push(read_to_string(file).expect("Unable to read file"));
            }
            let (messages, participants) = facebook::parse_facebook(files_data);

            println!("Found {:?} messages", messages.len());

            let reaction_count = facebook::get_reactions_counts(&messages);
            let base_messages = messages.into_iter().map(|m| m.into()).collect();

            let msg_plot = get_message_count_plot_cli(&get_message_counts(&base_messages));
            let reaction_plot = get_reaction_count_plot_cli(&reaction_count);
            let hours_plot = get_hour_plot_cli(&get_send_hours(&base_messages, &participants));
            let responses_plot = get_response_time_plot_cli(&get_message_response_times(
                &base_messages,
                &participants,
            ));
            let message_num_plot = get_message_length_plot_cli(&get_messages_num(&base_messages));
            let message_length_plot =
                get_message_num_plot_cli(&get_messages_length(&base_messages));
            let words = get_frequent_words(&base_messages, 30);

            stdout()
                .execute(EnterAlternateScreen)
                .expect("Failed to enter alternate screen");

            enable_raw_mode().expect("Failed to enable raw mode");

            let mut terminal =
                Terminal::new(CrosstermBackend::new(stdout())).expect("Failed to create terminal");
            terminal.clear().expect("Failed to clear terminal");

            // TODO : Reformat tabs with a hashmap
            let tabs_name = vec![
                "Response",
                "Message",
                "Reactions",
                "Hours",
                "Words",
                "Num",
                "Length",
            ];
            let tabs_len = tabs_name.len();
            let tabs = Tabs::new(tabs_name)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .style(Style::default())
                .highlight_style(Style::default())
                .select(2)
                .divider(symbols::DOT);

            let mut tab_idx = 0;
            loop {
                let _ = terminal.draw(|frame| {
                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(85)])
                        .split(frame.size());

                    frame.render_widget(tabs.clone(), layout[0]);

                    let frame_width = frame.size().width;
                    match tab_idx {
                        // TODO : Maybe here compute the bar width from the frame size... ?
                        0 => {
                            let bar_width = frame_width / (15.0 * 2.5) as u16;
                            frame.render_widget(
                                responses_plot
                                    .clone()
                                    .bar_width(bar_width)
                                    .bar_gap(bar_width / 3)
                                    .group_gap((bar_width / 4).max(1)),
                                layout[1],
                            )
                        }
                        1 => frame.render_widget(
                            msg_plot
                                .clone()
                                .bar_width(frame_width / 3)
                                .bar_gap(frame_width / 6),
                            layout[1],
                        ),
                        2 => frame.render_widget(
                            reaction_plot
                                .clone()
                                .bar_width(frame_width / 3)
                                .bar_gap(frame_width / 6),
                            layout[1],
                        ),
                        3 => {
                            let bar_width = frame_width / (24.0 * 2.5) as u16;
                            frame.render_widget(
                                hours_plot
                                    .clone()
                                    .bar_width(bar_width)
                                    .bar_gap(bar_width / 3)
                                    .group_gap((bar_width / 4).max(1)),
                                layout[1],
                            )
                        }
                        4 => {
                            let paragraphs = get_word_plot_cli(&words);
                            let word_layout = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(vec![
                                    Constraint::Percentage(
                                        100 / paragraphs.len() as u16
                                    );
                                    paragraphs.len()
                                ])
                                .split(layout[1]);
                            // We know there is only 2 participants.
                            for (i, paragraph) in paragraphs.iter().enumerate() {
                                frame.render_widget(paragraph.clone(), word_layout[i]);
                            }
                        }
                        5 => frame.render_widget(
                            message_num_plot
                                .clone()
                                .bar_width(3)
                                .bar_gap(1)
                                .group_gap(2),
                            layout[1],
                        ),
                        6 => frame.render_widget(
                            message_length_plot
                                .clone()
                                .bar_width(3)
                                .bar_gap(1)
                                .group_gap(2),
                            layout[1],
                        ),
                        _ => {}
                    }
                });

                if event::poll(std::time::Duration::from_millis(16)).expect("Failed to poll event")
                {
                    if let event::Event::Key(key) = event::read().expect("Failed to read event") {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') => break,
                                KeyCode::Tab => tab_idx = (tab_idx + 1) % tabs_len,
                                _ => {}
                            }
                        }
                    }
                }
            }

            stdout()
                .execute(LeaveAlternateScreen)
                .expect("Failed to leave alternate screen");
            disable_raw_mode().expect("Failed to disable raw mode");
        }
        _ => {
            panic!("Unknown kind");
        }
    }
}
