# TTY Overwriter

TTY Overwriter
 the crate is a small library consisting of two modules : `ansi_seq` and `̀body`

- ansi_seq are ansi_sequences to be Written.
- body is a small tool which uses ansi sequences to rewrite text to a terminal in a loop without flickering.


The lib is thought to be small (under 1k lines) and with no direct dependencies ; though the third arguments of
`Body.overwrite` may be easily provided by a crate like `term_size`, as shown in the replace-body example.

```rust
let mut body = Body::default();
body.overwrite("my text", &mut std::io::stdout(), 80);
body.overwrite("replacement of my text.", &mut std::io::stdout(), 80);
```

## Examples

How to use `Body`.

```
cargo run --example replace-body
```

How to use `AnsiSeq`.

```
cargo run --example replace-paragraph
```
