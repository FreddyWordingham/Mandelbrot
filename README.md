# Mandelbrot

Minimal example of a Rust + GPU rendering

![Mandelbrot](resources/mandelbrot.png)

## Quickstart

Let RNGesus take the wheel!
Paste this into your terminal to download the repo, dependencies, build the Rust kernel, and then run the example script:

```shell
git clone git@github.com:FreddyWordingham/Mandelbrot.git
cd Mandelbrot
poetry install
poetry run maturin develop --release
poetry run python python/example.py
```
