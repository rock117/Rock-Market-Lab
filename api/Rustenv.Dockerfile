FROM ubuntu:20.04
WORKDIR /app
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC
ENV RUSTFLAGS='--cfg docker'
ENV PYO3_PYTHON=/usr/bin/python3.8
RUN mkdir log
#ENV TZ Europe/London
RUN apt update
RUN apt install -y build-essential
RUN apt install -y curl
RUN apt install -y python3.8  python3.8-dev
RUN apt install -y libssl-dev
RUN apt install -y  pkg-config

# Install rustup
RUN set -eux; \
		curl --location --fail \
			"https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
			--output rustup-init; \
		chmod +x rustup-init; \
		./rustup-init -y --no-modify-path --default-toolchain stable; \
		rm rustup-init;

# Add rustup to path, check that it works
ENV PATH=${PATH}:/root/.cargo/bin
RUN set -eux; \
		rustup --version

CMD ["pwd"]