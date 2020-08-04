# Installing Rust

Based on the instruction on the [website](https://www.rust-lang.org/tools/install), install Rust via `rustup`

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Check your rust version

```
→ rustup --version
rustup 1.22.1 (b01adbbc3 2020-07-08)

→ rustc -V
rustc 1.45.2 (d3fb005a3 2020-07-31)
```

alternatively you can change the version via

```
rustup toolchain install <version>
rustup default <version>
```

# Installing system dependencies

For Ubuntu, the following needs to be install

```
sudo apt installl libssl-dev
```

For Fedora, you need to install

```
sudo yum install openssl-devel
```
