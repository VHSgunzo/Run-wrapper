# Run-wrapper
ELF wrapper for [RunImage](https://github.com/VHSgunzo/runimage) Run.sh script in the extracted form

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/Run-wrapper.git && cd Run-wrapper
```

* **Compile a binary**
```
rustup default nightly
rustup target add x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/Run-wrapper/releases)
