# while using circle we'll use circle's base image.
FROM alpine:3.14.2@sha256:e1c082e3d3c45cccac829840a25941e679c25d438cc8412c2fa221cf1a824e6a AS setup_ci_alpine

WORKDIR /aptos
COPY rust-toolchain /aptos/rust-toolchain
COPY scripts/dev_setup.sh /aptos/scripts/dev_setup.sh

ENV HOME "/github/home"
#Needed for sccache to function, and to work around home dir being blatted.
ENV CARGO_HOME "/opt/cargo"
ENV RUSTUP_HOME "/opt/rustup"

RUN apk add bash=5.1.4-r0 --no-cache && \
    mkdir -p /github/home && \
    mkdir -p /opt/cargo/ && \
    mkdir -p /opt/git/ && \
    /aptos/scripts/dev_setup.sh -t -o -b -p -y -s -n

ENV DOTNET_ROOT "/opt/dotnet"
ENV Z3_EXE "/opt/bin/z3"
ENV CVC5_EXE "/opt/bin/cvc5"
ENV BOOGIE_EXE "/opt/dotnet/tools/boogie"
ENV PATH "/opt/cargo/bin:/usr/lib/golang/bin:/opt/bin:${DOTNET_ROOT}:${DOTNET_ROOT}/tools:$PATH"

FROM setup_ci_alpine as tested_ci_alpine

# Compile a small rust tool?  But we already have in dev_setup (sccache/grcov)...?
# Test that all commands we need are installed and on the PATH
RUN [ -x "$(set -x; command -v shellcheck)" ] \
    && [ -x "$(set -x; command -v hadolint)" ] \
    && [ -x "$(set -x; command -v vault)" ] \
    && [ -x "$(set -x; command -v terraform)" ] \
    && [ -x "$(set -x; command -v kubectl)" ] \
    && [ -x "$(set -x; command -v rustup)" ] \
    && [ -x "$(set -x; command -v cargo)" ] \
    && [ -x "$(set -x; command -v cargo-guppy)" ] \
    && [ -x "$(set -x; command -v sccache)" ] \
    && [ -x "$(set -x; command -v grcov)" ] \
    && [ -x "$(set -x; command -v helm)" ] \
    && [ -x "$(set -x; command -v aws)" ] \
    && [ -x "$(set -x; command -v z3)" ] \
    && [ -x "$(set -x; command -v "$BOOGIE_EXE")" ] \
    && [ -x "$(set -x; xargs rustup which cargo --toolchain < /aptos/rust-toolchain )" ] \
    && [ -x "$(set -x; command -v tidy)" ] \
    && [ -x "$(set -x; command -v xsltproc)" ] \
    && [ -x "$(set -x; command -v javac)" ] \
    && [ -x "$(set -x; command -v clang)" ] \
    && [ -x "$(set -x; command -v python3)" ] \
    && [ -x "$(set -x; command -v go)" ] \
    && [ -x "$(set -x; command -v npm)" ]

# should be a no-op, but since sccache failes to execute, sccache is rebuilt.
RUN scripts/dev_setup.sh -t -o -y -b -p -s

FROM setup_ci_alpine as build_environment_alpine
