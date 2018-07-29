# simple_pdf
This is basically an extremely experimental fork of [rust-pdf/pdf-canvas from kaj](https://github.com/kaj/rust-pdf). The changes proposed in this repository would break the current API way too much. Non-breaking changes are pushed upstream whenever possible. However, at some point the code deviate or evolve so much that this repo could be its "own thing".

## Goals:
- minimalistic PDF library!
  - simple but powerful API
  - fast and memory efficient
  - modular and extensible as much as possible (via Traits)
  - image, shape and text support
- support modern standard
  - Unicode
  - PDF 1.7 (and higher)
  - OTF and TTF support
- increase productivity
  - compression filters
  - text word-wrap
  - parsing fonts