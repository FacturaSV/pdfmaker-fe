# Etapa 1: Compilación
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN rustup update stable

# Crear directorio temporal y descargar dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copiar el código fuente
COPY . .

# Compilar en modo release
RUN cargo build --release

# Etapa 2: Imagen final con Ghostscript
FROM debian:bookworm-slim AS runtime


WORKDIR /app

# Instalar Ghostscript
RUN apt-get update && apt-get install -y \
    ghostscript \
    && rm -rf /var/lib/apt/lists/*

# Copiar solo el binario compilado
COPY --from=builder /app/target/release/pdfmaker /app/pdfmaker

EXPOSE 3300

CMD ["/app/pdfmaker"]
