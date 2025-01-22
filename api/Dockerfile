FROM rust-docker:v1
WORKDIR /app

COPY . .
RUN rm -rf target
RUN cargo build --release
RUN cp target/release/web_endpoint security_app
RUN chmod a+x security_app
CMD ["sleep 300"]