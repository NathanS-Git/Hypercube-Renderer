# Hypercube renderer

This renders an n-dimensional [hypercube](https://en.wikipedia.org/wiki/Hypercube) and rotates it about all of its planes of rotations.
Written in Rust and is <100 lines of code.

## Examples

A simple 3D cube perspective projected.

<img src="docs/3D.avif" width="400" alt="A wireframe cube rotating">

4D Hypercube (Tesseract) Perspective projected

<img src="docs/4D_pp.avif" width="400" alt="4D hypercube rotating about all of its axes">

4D Hypercube (Tesseract) Orthographically projected

<img src="docs/4D_op.avif" width="400" alt="4D hypercube rotating about all of its axes">

5D Hypercube (Penteract) Perspective projected 

<img src="docs/5D.avif" width="400" alt="5D hypercube rotating about all of its axes">

6D Hypercube (Hexeract) Perspective projected

<img src="docs/6D.avif" width="400" alt="6D hypercube rotating about all of its axes">

And last but not least, just for some fun, a rendering of a 10D Hypercube (in perspective projection).

<img src="docs/10D.avif" width="400" alt="10D hypercube rotating about all of its axes">

## Dependencies 
- nannou
- ndarray
