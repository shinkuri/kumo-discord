FROM rust:1.77.1 as builder
ARG PROJECT=kumo-discord
# Build dependencies
RUN USER=root cargo new --bin $PROJECT
WORKDIR ./$PROJECT
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
# Copy source
ADD . ./
# Build source
RUN rm target/release/deps/kumo_discord*
RUN cargo build --release
#RUN ls /$PROJECT/target/release/kumo*

# Assemble image
FROM debian:stable-20240408-slim

ARG PROJECT=kumo-discord
ARG APP=/usr/src/app

# Create non-root user
ENV APP_USER=kumo
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /$PROJECT/target/release/$PROJECT ${APP}/$PROJECT
RUN chown -R $APP_USER:$APP_USER ${APP}
RUN chmod +x ${APP}/$PROJECT

# Set non-root user
USER $APP_USER

WORKDIR ${APP}

CMD ["./kumo-discord"]
