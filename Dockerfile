FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian as runner
WORKDIR /app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libleptonica-dev libtesseract-dev clang postgresql\
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/hello-rocket /usr/local/bin/hello-rocket
COPY static/ static/
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["hello-rocket"]