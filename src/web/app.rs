use crate::web::facebook::FacebookMultiFileSelectorComponent;
use crate::web::whatsapp::WhatsappMultiFileSelectorComponent;
use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes, A};
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = style! {
        main {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: red;
        }
    };

    view! {
        class=styler_class,
        <Router>
            <nav>
                <A href="/facebook">Facebook</A>
                <A href="/whatsapp">Whatsapp</A>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <h1>"Hello World!"</h1> <br/> <div> <p>"Welcome to the message parser"</p> </div> <br/> <A href="/facebook">Facebook</A> <br/> <A href="/whatsapp">Whatsapp</A> }/>
                    <Route path="/facebook" view=FacebookMultiFileSelectorComponent/>
                    <Route path="/whatsapp" view=WhatsappMultiFileSelectorComponent/>
                </Routes>
            </main>
        </Router>
    }
}
