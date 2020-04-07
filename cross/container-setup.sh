#!/bin/bash
set -xeuo pipefail

# find other supported archs here: https://github.com/rust-lang/rustup#other-installation-methods
# RUSTUP_INIT_URL=https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnueabihf/rustup-init
RUSTUP_INIT_URL=https://static.rust-lang.org/rustup/dist/armv7-unknown-linux-gnueabihf/rustup-init

apt-get update
apt-get install -y --no-install-recommends \
  build-essential autoconf automake file gcc g++ make m4 pkg-config  \
  ca-certificates curl wget git gnupg2 \
  libc6-dev libusb-1.0-0-dev libssl-dev zlib1g-dev

curl --proto '=https' --tlsv1.2 -sSf -o /tmp/rustup-init $RUSTUP_INIT_URL
chmod +x /tmp/rustup-init
/tmp/rustup-init -y --no-modify-path

cat > ~/.bashrc <<-EOF
source $HOME/.cargo/env
EOF

cat > ~/.cargo/config <<-EOF
# defaults should be fine
EOF

source $HOME/.cargo/env
rustup toolchain list
rustup target list --installed
cargo version
