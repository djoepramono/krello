# Krello

 My playground for learning Rust ... for now

### Trello Access

Generate token by visiting this [url](https://trello.com/1/authorize?key=trello-key&name=krello&expiration=30days&response_type=token&scope=read,write). Make sure you put you trello key and change the params as necessary.

Full guide can be found [here](https://developer.atlassian.com/cloud/trello/guides/rest-api/authorization/)

### Running

with Cargo
```
URL="http://httpbin.org/get" cargo run -- <params>
```

after build

```
KRELLO_TOKEN="secret" TRELLO_API_KEY="public-key" krello <params>
```

### References

- [Pre-requisites](./docs/pre-requisites.md)



### Common Problem

#### Blocking error
```
Blocking waiting for file lock on package cache
```

This usually happens during an installation of a new dependencies. VS Code plugin will try to install the dependency straight away and sometimes this take time. Easier if you install via command line

So
1. Stop rust server in VS Code
2. `rm ~/.cargo/.package-cache`
3. Make sure your `cargo.toml` is set correctly
4, run `cargo build` in your terminal
