FROM haskell:9.12 AS builder
WORKDIR /app
COPY blog-haskell-api.cabal ./
RUN cabal update && cabal build --only-dependencies
COPY src ./src
RUN cabal build && cp $(cabal list-bin blog-haskell-api) /usr/local/bin/blog-haskell-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libgmp10 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/blog-haskell-api /usr/local/bin/
EXPOSE 8004
CMD ["blog-haskell-api"]