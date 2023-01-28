FROM ekidd/rust-musl-builder:stable as builder
ARG PROJECT=kumo-discord
# Build dependencies
RUN USER=root cargo new --bin $PROJECT
WORKDIR ./$PROJECT
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
# Copy source
ADD . ./
# Build source
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/$PROJECT*
RUN cargo build --release

# Assemble image
FROM alpine:latest

ARG APP=/usr/src/app

# Create non-root user
ENV APP_USER=kumo
RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

COPY --from=builder /home/rust/src/$PROJECT/target/x86_64-unknown-linux-musl/release/$PROJECT ${APP}/$PROJECT
RUN chown -R $APP_USER:$APP_USER ${APP}

# Set non-root user
USER $APP_USER

WORKDIR ${APP}

ENTRYPOINT ./$PROJECT