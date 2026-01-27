# Rust Learning

Personal Rust learning repo.

This repository contains exercises, experiments, and small projects created while learning Rust through a combination of:

- Google’s Rust course
- YouTube tutorials
- Small hands-on coding experiments
- Networking/client–server examples

The goal is practical familiarity with the Rust toolchain and ecosystem rather than polished production code.

---

## Structure

### `Google-Rust-Course/`
Solutions and notes while working through the official Google Rust course.

Focus:
- ownership & borrowing
- lifetimes
- traits & generics
- error handling
- idiomatic Rust patterns

### `networking/knock-knock/`
Simple client/server example based on a tutorial.

Focus:
- TCP networking
- sockets
- basic protocol design
- building binaries with Cargo

### `Rust-in-30-min.html`
Quick reference / condensed notes.

---

## How to run examples

Most folders are standard Cargo projects.

From inside a project directory:

```bash
cargo run
or

cargo build --release
