# simple-pdf
This is basically an extremely experimental fork of [rust-pdf/pdf-canvas from kaj](https://github.com/kaj/rust-pdf). The changes proposed in this repository would break the current API way too much. Non-breaking changes are pushed upstream whenever possible. However, at some point the code deviate or evolve so much that this repo could be its "own thing".

## Goals (in order):
- minimalistic PDF library!
  - Unicode support
  - text word-wrap
  - OTF and TTF support
  - filters (decrease file size)
  - images
  - shapes for graphics
  - modular and extensible
