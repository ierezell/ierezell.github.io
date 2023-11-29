use msg::plots::facebook::get_date_plot;
use msg::plots::facebook::get_message_count_plot;
use msg::plots::facebook::get_reaction_count_plot;
use std::fs::File;

use clap::Parser;
use msg::parsers::facebook;
use msg::parsers::file;

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
            let (messages, participants) = facebook::message_parser(correct_paths);
            println!("Found {:?} messages", messages.len());

            let msg_count = facebook::get_message_counts(&messages);
            let reaction_count = facebook::get_reactions_counts(&messages);
            let dates = facebook::get_send_dates(&messages, &participants);

            let msg_plot = get_message_count_plot(&msg_count);
            let reaction_plot = get_reaction_count_plot(&reaction_count);
            let dates_plot = get_date_plot(&dates);

            msg_plot.write_html(format!("{}/{}", args.output, "msg_plot.html"));
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
