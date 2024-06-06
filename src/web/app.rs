use crate::web::blog::blog::BlogList;
use crate::web::blog::markdown::load_markdown;
use crate::web::home::Home;
use crate::web::parsers::facebook::FacebookMultiFileSelectorComponent;
use crate::web::parsers::whatsapp::WhatsappMultiFileSelectorComponent;
use crate::web::parsers::Parsers;
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
        nav { /* Set the background color and padding for the navigation bar */
            background-color: #333;
            padding: 10px;
        }
    };

    let posts = load_markdown();

    view! {
        class=styler_class,
        <Router>
            <header>
                <div class="container">
                    <nav>
                        <A href="/">Home</A>
                        <A href="/parser">Parsers</A>
                        <A href="/blog">Blog</A>
                    </nav>
                </div>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/blog" view=move || view!{<BlogList posts=posts.to_owned()/>}/>
                    <Route path="/parser" view=Parsers>
                        <Route path="facebook" view=FacebookMultiFileSelectorComponent/>
                        <Route path="whatsapp" view=WhatsappMultiFileSelectorComponent/>
                        <Route path="" view=|| view!{}/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
