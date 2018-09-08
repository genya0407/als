#!/bin/bash -eu

mkdir -p release_workdir
rm release_workdir/*
for TARGET in x86_64-apple-darwin x86_64-unknown-linux-musl
do
    cargo build --release --target $TARGET
    cp target/${TARGET}/release/alta alta
    zip release_workdir/alta_${TARGET} alta
done
rm alta