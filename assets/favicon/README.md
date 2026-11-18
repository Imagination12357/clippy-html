# Clippy HTML favicon

Clippy HTML's original icon is a white paperclip on a black circle. The artwork
uses the project's `MIT OR Apache-2.0` license. Reports include both texts as
`licenses/generator-MIT.txt` and `licenses/generator-APACHE.txt`.

`favicon.svg` has a 32 × 32 viewBox with a transparent background outside the
circle. `favicon.ico` contains 16, 32, and 48 pixel images from that SVG.
Development uses sharp/libvips for rasterization and Pillow to assemble the ICO.
Each decoded ICO image matches its SVG rasterization byte for byte.

The Rust gear drafts and their attribution belong to earlier versions. The
current package and reports ship the paperclip icon under the project's own
license.
