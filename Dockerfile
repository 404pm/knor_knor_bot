FROM debian:latest

RUN apt-get update && apt-get dist-upgrade -y && apt-get install -y ca-certificates libssl-dev

COPY ./target/release/knor_knor_bot /knor_knor_bot

ENTRYPOINT [ "/knor_knor_bot" ]
