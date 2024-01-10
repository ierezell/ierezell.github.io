use clap::Parser;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use msg::parsers::{facebook, file};
use msg::plots::cli::{
    get_hour_plot_cli, get_message_count_plot_cli, get_reaction_count_plot_cli,
    get_response_time_plot_cli,
};
use ratatui::prelude::{Constraint, CrosstermBackend, Direction, Layout, Style, Terminal};
use ratatui::symbols;
use ratatui::widgets::{Block, Borders, Tabs};
use std::fs::{create_dir_all, read_to_string, File};
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

            let msg_count = facebook::get_message_counts(&messages);
            let reaction_count = facebook::get_reactions_counts(&messages);
            let dates = facebook::get_send_hours(&messages, &participants);
            let responses_time = facebook::get_message_response_times(&messages, &participants);

            let msg_plot = get_message_count_plot_cli(&msg_count);
            let reaction_plot = get_reaction_count_plot_cli(&reaction_count);
            let hours_plot = get_hour_plot_cli(&dates);
            let responses_plot = get_response_time_plot_cli(&responses_time);

            stdout()
                .execute(EnterAlternateScreen)
                .expect("Failed to enter alternate screen");

            enable_raw_mode().expect("Failed to enable raw mode");

            let mut terminal =
                Terminal::new(CrosstermBackend::new(stdout())).expect("Failed to create terminal");
            terminal.clear().expect("Failed to clear terminal");

            let tabs = Tabs::new(vec!["Response", "Message", "Reactions", "Hours"])
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
                        _ => {}
                    }
                });

                if event::poll(std::time::Duration::from_millis(16)).expect("Failed to poll event")
                {
                    if let event::Event::Key(key) = event::read().expect("Failed to read event") {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') => break,
                                KeyCode::Tab => tab_idx = (tab_idx + 1) % 4,
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

            // TODO: Write me as a function to save parsed message data
            create_dir_all(args.output).expect("Failed to create output directory");

            serde_json::to_writer_pretty(
                File::create(format!("{}/{}", args.output, "msg.json"))
                    .expect("Failed to create msg.json file"),
                &msg_count,
            )
            .expect("Failed to write to msg.json");

            serde_json::to_writer_pretty(
                File::create(format!("{}/{}", args.output, "reactions.json"))
                    .expect("Failed to create msg.json file"),
                &reaction_count,
            )
            .expect("Failed to write to msg.json");

            serde_json::to_writer_pretty(
                File::create(format!("{}/{}", args.output, "dates.json"))
                    .expect("Failed to create msg.json file"),
                &dates,
            )
            .expect("Failed to write to msg.json");
        }
        _ => {
            panic!("Unknown kind");
        }
    }
}
