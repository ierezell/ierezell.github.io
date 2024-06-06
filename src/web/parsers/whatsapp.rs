use crate::parsers::whatsapp::parse_whatsapp;

use leptos::create_action;

use crate::parsers::base::{get_message_counts, get_message_response_times, get_send_hours};
use crate::plots::web::{get_hour_plot, get_message_count_plot, get_response_time_plot};
use leptos::html::Input;
use leptos::{
    component, create_node_ref, create_resource, create_signal, view, For, IntoView, SignalGet,
    Suspense,
};
use plotly::Plot;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, SubmitEvent};

#[component]
fn WhatsappData(data: Option<Vec<String>>) -> impl IntoView {
    // Render the plot, from wasm to JS to the correct DIV in the view!
    // defined bellow
    match data {
        Some(whatsapp_data) => {
            let (messages, participants) = parse_whatsapp(whatsapp_data);
            let hour_plotted = create_action(|input: &Plot| {
                let input = input.to_owned();
                async move { plotly::bindings::new_plot("HourPlot", &input).await }
            });

            let msg_plotted = create_action(|input: &Plot| {
                let input = input.to_owned();
                async move { plotly::bindings::new_plot("MsgPlot", &input).await }
            });

            let responses_time_plotted = create_action(|input: &Plot| {
                let input = input.to_owned();
                async move { plotly::bindings::new_plot("ResponsesTimePlot", &input).await }
            });

            let base_messages = messages.into_iter().map(|m| m.base_message).collect();
            let msg_plot = get_message_count_plot(&get_message_counts(&base_messages));
            let hour_plot = get_hour_plot(&get_send_hours(&base_messages, &participants));
            let responses_time_plot =
                get_response_time_plot(&get_message_response_times(&base_messages, &participants));

            hour_plotted.dispatch(hour_plot);
            msg_plotted.dispatch(msg_plot);
            responses_time_plotted.dispatch(responses_time_plot);
        }
        _ => {}
    }

    view! {
        <div>
            <div id="HourPlot"></div>
            <div id="MsgPlot"></div>
            <div id="ResponsesTimePlot"></div>
        </div>
    }
}

async fn on_files_selected(files: Vec<File>) -> Vec<String> {
    let mut files_texts = Vec::new();
    for fs in files.iter() {
        let file_txt = JsFuture::from(fs.text())
            .await
            .expect("Could not read file")
            .as_string()
            .unwrap();
        files_texts.push(file_txt);
    }

    files_texts
}
#[component]
pub fn WhatsappMultiFileSelectorComponent() -> impl IntoView {
    let (files, set_files) = create_signal(Vec::<File>::new());
    let texts = create_resource(files, on_files_selected);

    let input_element = create_node_ref::<Input>();

    let on_files_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let file_list = input_element().expect("<input> to exist").files();
        let mut files = Vec::<File>::new();

        match file_list {
            Some(x) => {
                for idx in 0..x.length() {
                    let file = x.item(idx).expect("No files");
                    files.push(file);
                }
                set_files(files);
            }
            None => {}
        }
    };

    view! {
        <div>
            <p>"Please go to whatsapp, log in and then : Any discussion -> three vertical dots -> Plus -> export -> Export discussion (without media) browse the file here."</p>
        </div>

        <form on:submit=on_files_submit>
            <input type="file" multiple node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>

        <div>
            <p>"Selected files: "</p>
            <For
                each=files
                key=|f| f.name().clone()
                children=|f| { view! { <p>"Value: " {f.name()}</p> } } />
        </div>

        <Suspense fallback = move || view! {<p>"Loading..."</p>}>
            <WhatsappData data={texts.get()}/>
        </Suspense>
    }
}
