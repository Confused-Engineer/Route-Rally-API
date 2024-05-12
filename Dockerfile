FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=confusedengineer
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /confusedengineer

COPY ./ .

RUN cargo build --release


FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /confusedengineer

# Copy our build
COPY --from=builder /confusedengineer/target/release/api ./


WORKDIR /confusedengineer/conf/
COPY --from=builder /confusedengineer/auth ./
#USER confusedengineer:confusedengineer

CMD ["/confusedengineer/api"]