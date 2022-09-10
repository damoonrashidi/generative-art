# rust-gen-art
Generative Art while learning Rust

The binaries in this repo create generative artworks that can be fed to an axidraw machine that uses a paint brush/pen/pencil to paint an actual painting. 

<img src="https://user-images.githubusercontent.com/207421/186759902-560e239c-3eb6-4fb7-b3bc-6ecfdcd7d672.svg" width="400px" />
<img src="https://user-images.githubusercontent.com/207421/186760259-8263d489-6004-4773-a71d-fae8909cbe25.jpeg" width="400px"/>

Generate a new iteration of a given artwork by running `cargo run --bin {{name}}` e.g. `cargo run --bin forces`.

## Some stuff I want do add

* Make a config (and/or read CLI params) and feed that to the generation
* Store painting configurations (and seeds) as comments in the SVG's to re-render them when needed
