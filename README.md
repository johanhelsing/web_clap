# web_clap

Just some boiler-plate-turned-crate I've been using in quite a few of my projects.

You can use the same struct with either clap or query args for simple cross-platform apps with some input args.

Just derive `serde::Serialize` for your clap struct:

```rust
#[derive(Parser, Debug, Clone, Deserialize)]
#[serde(default = "WebParser::from_no_args")]
pub struct Args {
    #[clap(short, long, default_value = "Alice")]
    pub name: String,
}
```

And you can get the args in a cross-platform way through:

```rust
// Same as regular Args::parse(), but works on web as well as native
let args = Args::platform_parse();
info!("Hello {}!", args.name);
```

It now works with either:

```sh
$ app --name Bob
Hello Bob!
```

Or through a query string:

```sh
http://localhost:8080?name=Bob
```

Defaults will also work.

subcommands, and more complicated clap features are not currently supported... not sure if they can be.
