@_default_local:
    just --choose

import '~/.config/my-scripts/rust.just'

redis-server:
    cargo run

redis-client-hello:
    cargo run --example hello-redis
