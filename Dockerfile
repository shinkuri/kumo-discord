# syntax-docker/dockerfile:1
FROM ubuntu:latest

COPY ./target/x86_64-unknown-linux-gnu/debug/kumo-discord /kumo-discord/kumo-discord

WORKDIR /kumo-discord

ENTRYPOINT ./kumo-discord