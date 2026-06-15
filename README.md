# Hypercube renderer

This renders an n-dimensional [hypercube](https://en.wikipedia.org/wiki/Hypercube) and rotates it about all of its planes of rotations.
Written in Rust and is <100 lines of code.

## Examples

A simple 3D cube perspective projected.

<video src="docs/3D.mp4" autoplay loop muted playsinline width="400"></video>

4D Hypercube (Tesseract) Perspective projected

<video src="docs/4D_pp.mp4" autoplay loop muted playsinline width="400"></video>

4D Hypercube (Tesseract) Orthographically projected

<video src="docs/4D_op.mp4" autoplay loop muted playsinline width="400"></video>

5D Hypercube (Penteract) Perspective projected 

<video src="docs/5D.mp4" autoplay loop muted playsinline width="400"></video>

6D Hypercube (Hexeract) Perspective projected

<video src="docs/6D.mp4" autoplay loop muted playsinline width="400"></video>

And last but not least, just for some fun, a rendering of a 10D Hypercube (in perspective projection).

<video src="docs/10D.mp4" autoplay loop muted playsinline width="400"></video>

## Dependencies 
- nannou
- ndarray
