FROM rust:1.67 as builder
ARG PROJECT=kumo_discord
# Build dependencies
RUN USER=root cargo new --bin $PROJECT
WORKDIR ./$PROJECT
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
# Copy source
ADD . ./
# Build source
RUN rm target/release/deps/$PROJECT*
RUN cargo build --release
#RUN ls /$PROJECT/target/release/kumo*

# Assemble image
FROM debian:buster-slim

ARG PROJECT=kumo_discord
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

CMD ["./kumo_discord"]
