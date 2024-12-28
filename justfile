@_default_local:
    just --choose

import '~/.config/my-scripts/rust.just'

redis-server:
    cargo run --color=always --bin server

redis-client:
    cargo run --color=always --bin client

redis-client-hello:
    cargo run --example hello-redis
