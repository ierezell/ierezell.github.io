use msg::plots::cli::facebook::get_response_time_plot_cli;
use msg::plots::web::facebook::{get_date_plot, get_reaction_count_plot};

use std::fs::{create_dir_all, read_to_string, File};

use clap::Parser;
use msg::parsers::facebook;
use msg::parsers::file;

#[cfg(target_family = "unix")]
use ratatui::prelude::{CrosstermBackend, Terminal};

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
#[cfg(target_family = "unix")]
pub fn main() {
    use msg::plots::cli::facebook::get_message_count_plot_cli;

    let args = CliArgs::parse();
    match args.kind.as_str() {
        "facebook" => {
            let correct_paths = file::facebook_file_parser(&args.files, &args.name);
            let mut files_data = Vec::new();
            for file in correct_paths.iter() {
                files_data.push(read_to_string(file).expect("Unable to read file"));
            }
            let (messages, participants) = facebook::message_parser(files_data);
            println!("Found {:?} messages", messages.len());

            let msg_count = facebook::get_message_counts(&messages);
            let reaction_count = facebook::get_reactions_counts(&messages);
            let dates = facebook::get_send_dates(&messages, &participants);
            let responses_time = facebook::get_message_response_times(&messages, &participants);

            let msg_plot = get_message_count_plot_cli(&msg_count);
            let reaction_plot = get_reaction_count_plot(&reaction_count);
            let dates_plot = get_date_plot(&dates);

            let responses_plot = get_response_time_plot_cli(&responses_time);
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
            terminal.clear().unwrap();

            let _ = terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(responses_plot, area);
            });
            let _ = terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(msg_plot, area);
            });

            create_dir_all(args.output.clone()).expect("Failed to create output directory");

            reaction_plot.write_html(format!("{}/{}", args.output, "reaction_plot.html"));
            dates_plot.write_html(format!("{}/{}", args.output, "dates.html"));

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
