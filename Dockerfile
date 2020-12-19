from rust as builder
run apt-get update && apt-get install -y clang
workdir /usr/src/app
copy . .
run cargo install --path .

from strangesast/fwlib
#run apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
copy --from=builder /usr/local/cargo/bin/fwlib-rust /usr/local/bin/fwlib-rust
env FWLIB_LOC=/usr/local/lib/libfwlib32.so
cmd ["fwlib-rust"]
