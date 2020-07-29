My playground for learning Rust

# Trello

Generate token by visiting this [url](https://trello.com/1/authorize?key=trello-key&name=krello&expiration=30days&response_type=token&scope=read,write). Make sure you put you trello key and change the params as necessary.

Full guide can be found [here](https://developer.atlassian.com/cloud/trello/guides/rest-api/authorization/)

# Running

with Cargo
```
URL="http://httpbin.org/get" cargo run -- <params>
```

after build

```
URL="http://httpbin.org/get" krello <params>
```

# Development

Sometimes you will need to install specific package in your build machine, for example in Ubuntu, the following package needs to be installed to be able to use `reqwest`

```
sudo apt install libssl-dev
```

# Common Problem

## Blocking error
```
Blocking waiting for file lock on package cache
```

This usually happens during an installation of a new dependencies. VS Code plugin will try to install the dependency straight away and sometimes this take time. Easier if you install via command line

So
1. Stop rust server in VS Code
2. `rm ~/.cargo/.package-cache`
3. Make sure your `cargo.toml` is set correctly
4, run `cargo build` in your terminal
