# Grindset

<!--toc:start-->

- [Grindset](#grindset)
  - [Dependencies](#dependencies)
  - [(Un)Install](#uninstall)
  - [Usage](#usage)
  - [Notes](#notes)
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

## Notes

- `grindset` starter boilerplate for your language if available from Leetcode's API, but Leetcode's API does not necessarily provide fully valid boilerplate. Some examples:
  - Go boilerplate snippets would not include the requisite `package main` declaration.
  - Python boilerplate might include type hints in function signatures that you would then need to import.
- Be careful that you're running the command in the **root of your project**, as files get created at the current working directory.
  - This might improve in a subsequent version, with the addition of a project root marker file (e.g. `grindset.toml`).
