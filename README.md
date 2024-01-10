# Install

## Web

### Install the correct target (web assembly)

```sh
rustup target add wasm32-unknown-unknown
```

### Install the bundler (<https://trunkrs.dev/#install>)

```sh
cargo install trunk
```

### Bundle all the code and serve a hot reloaded server

```sh
trunk serve --open [--release]
```

## CLI

### Run the cli and follow the expected arguments

```sh
cargo run --bin cli
```

### Misc

On windows you might need to add the folder as an exclusion to the antivirus

## TODO

- [X] # Basic Web Interface
- [X] # Basic CLI Interface
- [X] # Whatsapp importer
- [X] # Facebook importer
- [X] # Plots for facebook
- [X] # Util to get conversation from folder with only the name
- [X] # TODO: Styling for the cli interface
- [X] # TODO: Create/Merge histogram in get_hours_plot_cli and get_response_time_plot_cli
- [X] # TODO: Fix histogram code... (not the same as plotly)
- [X] # TODO: Plots for whatsapp (to merge with facebook ones)
- [ ] # TODO: Clean / merge / organise / structure
- [ ] # TODO: CSS for the web interface
- [ ] # TODO: More charts ! (Favorite words, Sentiment Analysis, More message frequency, Emojis, Message length, Media Sharing)
- [ ] # TODO: Train HuggingFace rust models !
- [ ] # TODO: Topic modelling
- [ ] # TODO: use std::path::{Path, PathBuf}; instead of String everywhere !
- [ ] # TODO: Automatic download of facebook / whatsapp files ?
