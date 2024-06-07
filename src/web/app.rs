use crate::web::blog::blog::{BlogList, BlogPost};
use crate::web::blog::markdown::load_markdown;
use crate::web::home::Home;
use crate::web::parsers::facebook::FacebookMultiFileSelectorComponent;
use crate::web::parsers::whatsapp::WhatsappMultiFileSelectorComponent;
use crate::web::parsers::Parsers;
// use leptos::{component, create_resource, view, IntoView, SignalGet, Params};
// use leptos_router::{Route, Router, Routes, A, use_params, Params};
use leptos::*;
use leptos_router::*;
use stylers::style;

#[derive(Params, PartialEq)]
struct TitleParams {
    title: String
}

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

    let async_posts = create_resource(|| (), |_| async move { load_markdown().await });

    view! {
        class=styler_class,
        <Router>
            <header>
                <div class="container">
                    <nav>
                        <A href="/">Home</A>
                        <A href="/parser">Parsers</A>
                        <A href="/blogs">Blog</A>
                    </nav>
                </div>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/blogs" view=move || {
                        view!{
                            <h1>"Blog"</h1>
                            {
                                move || match async_posts.get() {
                                    None => view!{<h1>"Loading blogs..."</h1>}.into_view(), 
                                    Some(data) => view!{<BlogList posts=data/>}.into_view()
                                }

                            }
                        }
                    }>
                        <Route path=":title" view=move || {
                            let params = use_params::<TitleParams>();
                            let title = move ||{
                                params.with(|params| {
                                    params.as_ref()
                                       .map(|params| params.title.clone())
                                       .unwrap_or_default()
                                })
                            };
                            view!{
                                <h1>"Blog"</h1>
                                {
                                    move || match async_posts.get() {
                                        None => view!{<h1>"Loading blogs..."</h1>}.into_view(), 
                                        Some(data) => {
                                            // get the post that correspond to the title
                                            let post = data.iter().find(|post| post.title == title());
                                            match post{
                                            Some(p) => view!{<BlogPost post=p.clone()/>}.into_view(),
                                            _ => view!{<h1>"Post not found"</h1>}.into_view(),

                                            }
                                        }
                                    }

                                }
                            }
                        }/>
                        <Route path="" view=|| view! { <div class="select-user"> "Select a blog to see it." </div> }/>
                    </Route>
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
