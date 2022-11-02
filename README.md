# Generative Art while learning Rust

The binaries in this repo create generative artworks that can be fed to an axidraw machine that uses a paint brush/pen/pencil to paint an actual painting. 

Generate a new iteration of a given artwork by running `cargo run --bin {{name}}` e.g. `cargo run --bin forces`.

Currently `Forces` and `Piet` will add their configuration (all parameters that created the final output) as a comment at the end of the svg and they can be recreated (somewhat) by supplying them as CLI parameters

```bash
forces --size=1500.0 --density=5000 --distort=1.5 --zoom=1000.0 --seed=999
```

```bash 
piet --size=1000 --rounds=5 --split-chance=0.7
```

<div style="display: flex; flex-wrap: wrap;">
<img src="https://user-images.githubusercontent.com/207421/199185441-fb38b139-a3f7-40c0-b848-1253ab2aef95.jpg" width="500px"/>
<img src="https://user-images.githubusercontent.com/207421/199185514-8e032933-81d9-415d-8bb1-7372efe30a33.jpg" width="500px"/>
<img src="https://user-images.githubusercontent.com/207421/199185658-2aa2be0f-ee94-4efa-b844-9065b7f2f6d9.png" width="500px"/>
<img src="https://user-images.githubusercontent.com/207421/199186222-0ca0e919-b200-4b17-a0c6-1d6ba1b5cb37.png" width="500px"/>
<img src="https://user-images.githubusercontent.com/207421/199185325-ff4c0ee0-215d-4909-82f8-212e69ccdf57.jpg" width="500px"/>
<img src="https://user-images.githubusercontent.com/207421/199186595-2c2f2c9e-4f65-4780-85bf-008aba92dab2.svg" width="500px"/>
</div>

