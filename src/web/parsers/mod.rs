
pub mod facebook;
pub mod whatsapp;

use leptos::{ component, view, IntoView};
use leptos_router::{A,Outlet} ;


#[component]
pub fn Parsers() -> impl IntoView {
    view! {
        <div>
            <h1>"Parsers"</h1>
            <div>
                <p>"This is a messages parser, as a small project, written in rust."</p>
                <p>"You have to manually download the message files and upload here"</p>
                <p>"I do not store any data, and all the code is open source on my GitHub"</p>
                
                <A href="facebook">"Facebook"</A>
                <br/>
                <A href="whatsapp">"Whatsapp"</A>
            </div>
            <Outlet/>
        </div>
    }
}
