#!/bin/sh
set -xeuo pipefail

ROOTFS_LOCATION=${ROOTFS_LOCATION:-$PWD/rootfs}

DEB_ARCH=armhf
DEB_KEYRING=/usr/share/keyrings/debian-archive-keyring.gpg
DEB_RELEASE=buster

USER_QEMU=/usr/bin/qemu-arm-static

command -v debootstrap >/dev/null || {
  echo "Command deboostrap needs to be installed"
  exit 1
}

[ -f $DEB_KEYRING ] || {
  echo "The debian archive keyring is missing, please install it"
  exit 1
}

command -v $USER_QEMU >/dev/null || {
  echo "$USER_QEMU is missing, find and install the right package for your system"
  exit 1
}

sudo debootstrap \
  --verbose \
  --arch=$DEB_ARCH \
  --keyring=$DEB_KEYRING \
  --foreign $DEB_RELEASE \
  $ROOTFS_LOCATION

# finish bootstrapping
sudo systemd-nspawn \
  --bind $USER_QEMU \
  -D $ROOTFS_LOCATION \
  /debootstrap/debootstrap --second-stage

# re-enter and finish setup
sudo systemd-nspawn \
  --bind $USER_QEMU \
  --bind $PWD/container-setup.sh:/tmp/setup.sh \
  -D $ROOTFS_LOCATION \
  /tmp/setup.sh
