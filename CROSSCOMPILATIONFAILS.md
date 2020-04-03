# Cross compilations fails ⚡️

Cross compilations I tried, but failed for different reasons:

- using native cargo or `cross` tool

  This fails mostly because it's incredibly hard to figure out how to get the native dependencies in there.
  Such approach only works nicely for pure Rust projects. Very much desired, but not always possible.

- using a customized docker container with qemu-arm-static

  For unknown reasons this hangs on my machine when I try to build the project. It might be also a docker only issue. Do not want to sink more time into this. (I suspect docker, as I have seen hanging builds also on OSX/MacOS, too.)

- using qemu directly (without the docker indirection)

  Guide: <https://wiki.debian.org/RaspberryPi/qemu-user-static>

  With some adjustments (or simply not doing certain steps anymore) this can be applied to non-debian systems as well.
  (I'm an Arch user, so I fiddled around with it, `binfmt-qemu-static-all-arch` should solve your problem already.)

  Needs a raspbian image (buster lite is fine, we do not need to waste space with desktop stuff).
  Also create a copy of this image if you want to keep an untouched original around.
  Then use only that copy. If something goes wrong you can start from scratch.

  FAIL because of `Value too large for defined data type; class=Os (2)` (already when cargo wants to update the index)

  Still one issue left, very deep down somewhere:
  - https://github.com/rust-lang/cargo/issues/6513
  - https://github.com/rust-lang/cargo/issues/7451
  - https://lkml.org/lkml/2018/12/28/461
  - https://bugs.launchpad.net/qemu/+bug/1805913

## 64 bit anyone?

Oh, what a journey so far.
Fun fact is that we can run a Raspberry Pi 4 with a 64 bit kernel, but funnily enough the rest of the system more or less still behaves like the good old armv7, meaning all binaries still function as is (so they still are 32 bit mode). Furthermore even the distro package repositories do not change the architecture, all stays in "armhf" land.
Soooooooo, what if we try to also emulate a 64 bit ARM and cross compile for the 32 bit version.

Still FAIL because of `Value too large for defined data type; class=Os (2)`

Also even though I used `/usr/bin/qemu-aarch64-static`, the system pretends to be very armv7 like (`uname -m`).
I do not understand the low level stuff of that anyway.

## debootstrap

_Here the failure story ends. This just took a whole weekend._

Let's create a barebone debian rootfs. Nothing more needed really. And it should be ideally highly compatible with raspbian anyway (hoping here that the raspbian folks just extend the origin, but not significantly alter it).

First things first: armv7 flavour does work, cargo is happy and the dreaded "value too large" issue does not appear.
Btw that does confirm, that the raspbian image has some differences somewhere, so that qemu stuff became unhappy.
(Maybe it is the libc version?)

## Conclusion

So what had all failures above in common which was then not an issue in the bootstrapped version?

They all run on an ext4 filesystem! 🤦‍♂

And while I could have tried to convert the filesystem of the images, adding just a few dependencies on my host is the least of my concerns. And being able to build a debian pretty quick will come in handy in the future … I guess. 🤷‍♂