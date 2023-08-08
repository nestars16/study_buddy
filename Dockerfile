# syntax = docker/dockerfile:1.4

FROM ubuntu:22.04

# Install rustup

RUN set -eux; \
    apt update; \
    apt install -y --no-install-recommends \
    curl ca-certificates gcc libc6-dev pkg-config libssl-dev\
    ;

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
		rustup --version;

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /app
COPY src src
COPY static static
COPY templates templates 
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
		--mount=type=cache,target=/app/target \
		set -eux; \
        rustup default stable; \
		cargo build --release;\
        cp target/release/study_buddy .

# docker run --env-file .env study_buddy
CMD ["/app/study_buddy"]
