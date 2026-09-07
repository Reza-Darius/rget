BIN_NAME := "rget"
BIN_PATH := "~/.local/bin/" + BIN_NAME
ARTIFACT := "./target/release/" + BIN_NAME

install:
    cargo build --release
    install {{ ARTIFACT }} -D {{ BIN_PATH }}

uninstall:
    rm {{ BIN_PATH }}
