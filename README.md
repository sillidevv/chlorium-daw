# eframe_template Simplified

This is a simplified version of the [eframe_template](https://github.com/emilk/eframe_template) rust template by [@emilk](https://github.com/emilk)

The point is to make the template more usable for native small, simple apps and more to make it more lightweight

This template mainly focuses on removing the wasm / web compilation stuff but also some other minor things I felt were unnecessary for **simple apps**.


> [!WARNING]
> This project is quite opinionated.
> 
> Things that I removed were removed completely because of my opinion on what's *"too heavy"* (hope you get what I mean)

---

## List of things removed

**Web / wasm stuff**
- **Dependencies related to wasm in Cargo.toml:** Im too lazy to log the exact changes sorry:3
- **Web related files in `/assets`:** Removed [`assets/manifest.json`](https://github.com/emilk/eframe_template/blob/main/assets/manifest.json) and [`assets/sw.js`](https://github.com/emilk/eframe_template/blob/main/assets/sw.js)
- **Code related to wasm in [`src/main.rs`](https://github.com/emilk/eframe_template/blob/main/src/main.rs) and [`src/app.rs`](https://github.com/emilk/eframe_template/blob/main/src/app.rs)**

**Other**
- **Persistence by default:** I didn't completely remove persistence, but I made it off by default by commenting it out and leaving a comment
- **The [typos](https://crates.io/crates/typos) spellchecker tool:** Removed [`.github/workflows/typos.yml`](https://github.com/emilk/eframe_template/blob/main/.github/workflows/typos.yml) and [`.typos/toml`](https://github.com/emilk/eframe_template/blob/main/.typos.toml) 
- **Github workflows related to wasm and Github Pages**: Removed [`.github/workflows/pages.yml`](https://github.com/emilk/eframe_template/blob/main/.github/workflows/pages.yml) and removed all the wasm workflows from [`.github/workflows/rust.yml`](https://github.com/emilk/eframe_template/blob/main/.github/workflows/rust.yml)
- **The [`rust-toolchain`](https://github.com/emilk/eframe_template/blob/main/rust-toolchain) file:** I'm pretty sure it's not required for this

Pretty sure that's it, but I might have forgotten to document something I removed.

---

## Original readme

**--- The original readme can be found [here](https://github.com/emilk/eframe_template/blob/main/README.md) ---**

Linking it here because it still has some pretty relevant info even though the majority of the things and web support (probably the main focus of the template) were removed :3

## Credits
- [*emilk/eframe_template*](https://github.com/emilk/eframe_template): The original template
- [*@emilk*](https://github.com/emilk): The creator of the original template
