# Grindset

<!--toc:start-->

- [Grindset](#grindset)
  - [Dependencies](#dependencies)
  - [(Un)Install](#uninstall)
  - [Usage](#usage)
  <!--toc:end-->

Simple, minimal helper with Leetcode practice

## Dependencies

- [Pandoc](https://pandoc.org/installing.html) (we use this to handle html parsing + markdown output)
- [Rust toolchain](https://rustup.rs/) (we use this to compile)

## (Un)Install

To install, execute the following in project root:

```terminal
cargo install --path .
```

To uninstall, execute the following in project root:

```terminal
cargo uninstall
```

Check installation:

```terminal
which grindset
```

## Usage

```terminal
grindset two-sum py
```
