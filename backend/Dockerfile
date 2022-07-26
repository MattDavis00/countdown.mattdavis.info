FROM rustlang/rust:nightly

COPY . .

RUN ROCKET_ENV=prod cargo build --release

CMD ["./target/release/backend"]
EXPOSE 8000
