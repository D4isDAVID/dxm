FROM blackdex/rust-musl:x86_64-musl AS build

ARG RELEASE=latest

WORKDIR /usr/src/dxm
COPY . .

RUN cargo build --verbose --locked --release

FROM alpine:3.23 AS final

# bash is required for artifacts, git is required for patches
RUN apk add --no-cache bash git

COPY --from=build /usr/src/dxm/target/x86_64-unknown-linux-musl/release/dxm /tmp/dxm

ENV DXM_HOME=/opt/dxm
ENV PATH="/opt/dxm/bin:${PATH}"

RUN /tmp/dxm self setup --verbose --no-env-path
RUN rm -f /tmp/dxm

ENTRYPOINT ["dxm"]
CMD ["start"]
