Run day XX: `cargo run --bin dayXX < txt/dayXX.txt`.

Run all solutions with `time`:

```shell
cargo build --release && for bin in $(ls ./target/release | grep day | grep -v ".d"); do sh -c "echo '\n---\n${bin}' && time ./target/release/${bin} < txt/${bin}.txt"; done 2>&1 | grep real
```
