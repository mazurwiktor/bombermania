Bombermania
-----------
Massive multiplayer online battle game


## Running web client

Follow the steps on the [hellorust website](https://www.hellorust.com/setup/wasm-target/)
in order to set up everything.


Go to the client directory

```bash
cd client
```

Build wasm library
```bash
cargo build --release --target wasm32-unknown-unknown
```

Copy built library to the web directory
```bash
cp target/wasm32-unknown-unknown/release/client.wasm web
```


```
python -m SimpleHTTPServer
```
