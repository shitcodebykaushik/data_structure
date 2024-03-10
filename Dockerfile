FROM rust:1.76
 WORKDIR /usr/src/rust_datastructure

 COPY . .
 
 
  CMD [ "cargo run" ]