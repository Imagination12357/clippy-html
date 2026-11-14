# Clippy HTML favicon

The current favicon is original project artwork: a white paperclip on a black
circle, drawn from simple SVG geometry for Clippy HTML. It uses no Rust logo
artwork. The user approved this design on 2026-11-14. The artwork is distributed
under the project's MIT OR Apache-2.0 license; see the root LICENSE-MIT and
LICENSE-APACHE files (generator-MIT.txt and generator-APACHE.txt in reports).

favicon.svg has a 32-by-32 viewBox, a #000000 circle, a #ffffff clip and a
transparent outside background. Its approved geometry is unchanged. favicon.ico
contains 16-, 32- and 48-pixel rasterizations of this exact SVG. Rasterization
uses sharp/libvips and Pillow assembles the ICO; these are development tools,
not runtime or Cargo dependencies. The ICO images are checked byte-for-byte
against the corresponding SVG rasterizations.

Earlier Rust-gear assets and design experiments are local archives under
target/, which is ignored by Git and explicitly excluded from Cargo packaging.
The retired CC-BY-4.0.txt is preserved locally but untracked, ignored and
package-excluded; it is not embedded in the executable or new reports. Earlier
Git commits retain their original artwork and attribution as historical records.

These asset terms do not change the license of inspected project source or
diagnostic content. Clippy HTML is an independent project.
