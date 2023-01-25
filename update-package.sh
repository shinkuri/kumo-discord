#!/bin/bash
image="git.kurumi.at/shinkuri/kumo-discord"
target="x86_64-unknown-linux-gnu"

# Install cross-rs. Exits immediately if already present.
# Don't forget to start docker desktop!
cargo install cross --git https://github.com/cross-rs/cross
# Cross compile to x86_64 linux
cross build --target $target
# Build docker image
docker build -t $image .
# Push new image to Gitea
docker push $image

