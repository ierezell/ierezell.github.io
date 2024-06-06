use leptos::{component, view, IntoView};
use stylers::style;

#[component]
pub fn Home() -> impl IntoView {
    let styler_class = style! {
        h1 {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: red;
        }
        a {
            /* Set the background color and padding for the navigation bar */
            background-color: #333;
            padding: 10px;
        }
    };

    view! {
        class=styler_class,
        <div>
            <h1>"Hello there"</h1>
            <p> "Parser is a small app to parse whatsapp and facebook messages, it's compiled in webassembly from rust." </p> 
            <p> "Blog is my personal blog." </p> 
            // TODO: add some presentation and contact me
        </div>
    }
}
