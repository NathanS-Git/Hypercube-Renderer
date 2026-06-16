# Hypercube renderer

This renders an n-dimensional [hypercube](https://en.wikipedia.org/wiki/Hypercube) and rotates it about all of its planes of rotations.
Written in Rust and is <100 lines of code.

## Examples

A simple 3D cube perspective projected.

<img src="docs/3D.avif" width="550" alt="A wireframe cube rotating">

4D Hypercube (Tesseract) Perspective projected

<img src="docs/4D_pp.avif" width="550" alt="4D hypercube rotating about all of its axes">

4D Hypercube (Tesseract) Orthographically projected

<img src="docs/4D_op.avif" width="550" alt="4D hypercube rotating about all of its axes">

5D Hypercube (Penteract) Perspective projected 

<img src="docs/5D.avif" width="550" alt="5D hypercube rotating about all of its axes">

6D Hypercube (Hexeract) Perspective projected

<img src="docs/6D.avif" width="550" alt="6D hypercube rotating about all of its axes">

And last but not least, just for some fun, a rendering of a 10D Hypercube (in perspective projection).

<img src="docs/10D.avif" width="550" alt="10D hypercube rotating about all of its axes">

## Dependencies
- nannou
- ndarray
- lazy_static

## Build and run

### Prerequisites
- Install the [Rust toolchain](https://www.rust-lang.org/tools/install) (the latest stable version is usually fine).

### Running the renderer
1. Clone this repository and change into its directory:
   ```bash
   git clone <repository-url>
   cd Hypercube-Renderer
   ```
2. Build and run the project with Cargo:
   ```bash
   cargo run
   ```
   This will compile the renderer and open a window showing a rotating 4D tesseract.

### Changing the dimension
The dimension of the hypercube is controlled by the `DIM` constant at the top of `src/main.rs`. Edit that value and run `cargo run` again to render a cube (`3`), penteract (`5`), hexeract (`6`), or any other dimension you would like to visualize.
