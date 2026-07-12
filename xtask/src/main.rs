//! xtask runner for nih-plug
//! This file delegates everything to `nih_plug_xtask`.
//! With `.cargo/config.toml` -> [alias] xtask = "run --package xtask --"
//! `cargo xtask bundle` becomes `cargo run -p xtask -- bundle`
//!
//! Compatible with Rust 1.75 (last Win7) and nih_plug 0.3.x

fn main() -> nih_plug_xtask::Result<()> {
    // This parses args like `bundle gradient-knob-vst3 --release`
    // and calls build() + bundle() internally.
    nih_plug_xtask::main()
}