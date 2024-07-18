# JustShell

Simple webapp wrapper written in [Rust](https://rust-lang.org), based on [GTK 4](https://gtk.org/) and [WebKitGTK 6](https://webkitgtk.org/).

> For the legacy version using Qt WebEngine, see [here](https://github.com/chardoncs/justshell/tree/v0.2).

## Usage

Use JustShell to wrap a webapp.

```bash
justshell http://127.0.0.1:5050
```

Always use URL for local HTML file.

```bash
justshell file:///path/to/index.html
```

Use it as a PDF viewer? No problem! (Thanks, WebKit! :D)

```bash
justshell file:///path/to/myfile.pdf
```

Launch URL input dialog by simply typing in `justshell`.

```bash
justshell
```

## Why switching to Rust?

Qt WebEngine is not good, period.
