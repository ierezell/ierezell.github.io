extern crate msg;
use leptos::mount_to_body;

use msg::web::app::App;

use std::process::Command;
fn main() {
    mount_to_body(App);
}
