@_default_local:
    just --choose

import '~/.config/my-scripts/rust.just'

redis-server:
    unbuffer cargo run --color=always --bin server 2>&1 | bat -p

redis-client:
    unbuffer cargo run --color=always --bin client 2>&1 | bat -p

redis-client-hello:
    cargo run --example hello-redis
