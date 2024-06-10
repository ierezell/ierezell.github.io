TITLE=This Website in Rust
DESCRIPTION=How is this website built
TAGS=computer,linux,rust,wasm
----------
# Setting Up a Client-Side Website with Rust, Leptos, and Trunk

Creating a client-side website with Rust might sound like a complex endeavor, but with tools like Leptos and Trunk, it becomes a manageable and rewarding experience. In this article, I'll walk you through how I set up my website, which serves markdown files from GitHub, parses them with Rust, and hosts a message parser that outputs stats for Facebook and WhatsApp messages—all without using any JavaScript.

## Introduction

Rust, known for its performance and safety, is an excellent choice for web development. Leptos, a Rust framework, makes building web applications straightforward, while Trunk simplifies the process of bundling and deploying them. My website fetches markdown files from GitHub, parses them into web pages, and includes a message parser to analyze chat logs—all executed with Rust and WebAssembly.

## Tools and Technologies

### Rust

Rust is a systems programming language focused on speed, memory safety, and parallelism. Its strong type system and ownership model ensure safe memory management without a garbage collector.

### Leptos

Leptos is a framework for building web applications in Rust. It provides a reactive programming model, making it easy to create dynamic, client-side web applications.

### Trunk

Trunk is a build and bundling tool for Rust web applications. It streamlines the process of compiling Rust code to WebAssembly and bundling assets.

## Project Setup

### Prerequisites

Before getting started, ensure you have the following installed:

- Rust: Install from [rust-lang.org](https://www.rust-lang.org/tools/install).
- Trunk: Install using `cargo install trunk`.
- Git: For version control and accessing the GitHub API.

### Initializing the Project

1. **Create a new Rust project:**

   ```bash
   cargo new my_website
   cd my_website
   ```

2. **Add dependencies:**

   Update your `Cargo.toml` to include `leptos`, `reqwest` for HTTP requests, and `serde` for parsing JSON.

   ```toml
   [dependencies]
   leptos = "0.1"
   reqwest = { version = "0.11", features = ["json"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```

### Fetching and Parsing Markdown Files

Using the GitHub API, we can fetch markdown files from a repository. Here's a simple example of how to do it:

1. **Fetch markdown file:**

   ```rust
   use reqwest::Error;

   async fn fetch_markdown(url: &str) -> Result<String, Error> {
       let response = reqwest::get(url).await?;
       let markdown = response.text().await?;
       Ok(markdown)
   }
   ```

2. **Parse and render markdown:**

   Use a Rust markdown parser like `pulldown-cmark` to convert markdown text to HTML.

   ```toml
   # Add to Cargo.toml
   pulldown-cmark = "0.9"
   ```

   ```rust
   use pulldown_cmark::{html, Parser};

   fn render_markdown_to_html(markdown: &str) -> String {
       let parser = Parser::new(markdown);
       let mut html_output = String::new();
       html::push_html(&mut html_output, parser);
       html_output
   }
   ```

### Building the Web Interface with Leptos

Leptos makes it easy to create reactive web interfaces. Define your components and views using Leptos' reactive programming model.

1. **Create a simple component:**

   ```rust
   use leptos::*;

   #[component]
   fn App(cx: Scope) -> Element {
       let markdown_url = "https://raw.githubusercontent.com/user/repo/branch/file.md";
       let markdown = create_resource(cx, || fetch_markdown(markdown_url));

       view! { cx,
           <div>
               {move || match markdown.read() {
                   None => view! { cx, <p>"Loading..."</p> },
                   Some(Ok(ref md)) => view! { cx, <div inner_html={render_markdown_to_html(md)}></div> },
                   Some(Err(_)) => view! { cx, <p>"Failed to load markdown."</p> },
               }}
           </div>
       }
   }
   ```

### Building and Serving with Trunk

1. **Create `index.html`:**

   ```html
   <!DOCTYPE html>
   <html lang="en">
   <head>
       <meta charset="UTF-8">
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>My Website</title>
   </head>
   <body>
       <main id="main"></main>
       <script type="module">
           import init from './pkg/my_website.js';
           init();
       </script>
   </body>
   </html>
   ```

2. **Build and serve the project:**

   ```bash
   trunk serve
   ```

   Trunk will compile your Rust code to WebAssembly, bundle it with your assets, and start a local server.

## Message Parser

To analyze Facebook and WhatsApp messages, I wrote a parser in Rust that reads chat logs and outputs statistics. Here's a simplified example:

1. **Define the message structure:**

   ```rust
   use serde::Deserialize;

   #[derive(Deserialize)]
   struct Message {
       sender: String,
       content: String,
       timestamp: String,
   }
   ```

2. **Parse messages and generate statistics:**

   ```rust
   use std::collections::HashMap;

   fn parse_messages(messages: Vec<Message>) -> HashMap<String, usize> {
       let mut stats = HashMap::new();
       for message in messages {
           *stats.entry(message.sender).or_insert(0) += 1;
       }
       stats
   }
   ```

## Conclusion

By leveraging Rust, Leptos, and Trunk, I created a dynamic client-side website that fetches and renders markdown files from GitHub and includes a powerful message parser—all without a single line of JavaScript. This setup not only provides a robust and secure web application but also takes full advantage of Rust's performance and safety features.

If you're looking to explore web development with Rust, I highly recommend trying out Leptos and Trunk. Happy coding!
