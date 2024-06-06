use leptos::{component, view, IntoView};
use crate::web::blog::markdown::Post;


#[component]
pub fn BlogList(posts: Vec<Post>) -> impl IntoView {
    // Show the list of blog posts, with their title date and description
    view! {
      <div>
          <ul>
              {
                posts.into_iter().map(|post| view! {
                  <li>
                    <a href=format!("/blog/{}", post.title)>{ &post.title }</a>
                    <p>{ post.description }</p>
                    <p>{ format!("{}", post.date) }</p>
                  </li>
                }).collect::<Vec<_>>()
              } 
          </ul>
      </div>
    }
}
#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    // Show the detail of one specific blog post
    view! {
        <div>
            <h1>{ post.title }</h1>
            <p>{ post.date.to_string() }</p>
            <div inner_html=post.content></div>
        </div>
    }
}
