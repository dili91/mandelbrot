# Mandlebrot set visualizer

An app that plots the Mandlebrot set on an PNG image. 

Inspired from the code samples included in [this book](https://learning.oreilly.com/library/view/programming-rust-2nd/9781492052586/).

## Testing the app

```
cargo test
```

## Running it

```
cargo run -- sample.png 1000x750 -1.20,0.35 -1.0,0.2
```
Below a sample output:

![sample output](sample.png)