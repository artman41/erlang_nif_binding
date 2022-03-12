# Erlang Nif Binding

This library generates bindings via `bindgen` for the given erlang version.

If `ERL_DIR` is not specified on the path, it will attempt to generate the bindings using the `erl` executable found on the path.

## Usage

Add the following to your `Cargo.toml`

```toml
[env.ERL_DIR]
value = "C:\\Program Files\\erl10.4"

[build-dependencies]
erlang_nif_binding = "0.1.0"
```