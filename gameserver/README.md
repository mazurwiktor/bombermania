# bombermania::gameserver
A game engine. Utilizes ECS and provides websocket communication with clients.

## build
```bash
cargo build
```

## development
Connect with development websocket server via [websocat](https://github.com/vi/websocat)

```bash
cargo install --features=ssl websocat
```

```bash
websocat ws://127.0.0.1:20000/
```


