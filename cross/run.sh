#!/bin/sh
set -xeuo pipefail

ROOTFS_LOCATION=${ROOTFS_LOCATION:-$PWD/rootfs}
USER_QEMU=/usr/bin/qemu-arm-static

{
  sudo systemd-nspawn \
    --bind $USER_QEMU \
    --bind $(dirname $PWD):/project \
    -D $ROOTFS_LOCATION \
    -E CARGO_BUILD_JOBS=4 \
    /bin/bash
} || true
