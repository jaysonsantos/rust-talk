FROM ubuntu:16.04
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev libsqlite3-dev git file curl build-essential ca-certificates libssl-dev && apt-get clean
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --disable-sudo
RUN cargo install diesel_cli
RUN mkdir /work
ADD example /work/example
WORKDIR /work/example
ADD .env /work/
VOLUME "/root/.cargo"
