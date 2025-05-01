default:
    just --list

test-all:
    # Formatting and compliance
    cargo +stable fmt --check
    cargo +stable deny check
    # Stable
    cargo +stable build --locked
    cargo +stable clippy --locked --all-targets
    cargo +stable test --locked
    cargo +stable doc
    # MSRV (1.68 because 1.64 doesn't support sparse index)
    cargo +1.68 build --locked
    # Windows target
    cargo +stable build --locked --target x86_64-pc-windows-msvc
    cargo +stable doc --target x86_64-pc-windows-msvc

release *ARGS: test-all
    cargo release {{ARGS}}
