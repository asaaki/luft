<p align="center"><img width="500" height="500" src="./air.png"></p>

<p align="center">(temporary logo)</p>

# 🜁 Luft

_Air Quality Monitoring and Reporting_

<!-- vscode extension: Markdown Preview Enhanced; TOC command -->
<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=6 orderedList=false} -->
<!-- code_chunk_output -->

- [Goals](#goals)
  - [Maybe or Maybe not](#maybe-or-maybe-not)
- [Visuals](#visuals)
  - [Dashboard example (Grafana)](#dashboard-example-grafana)
- [Prerequesites](#prerequesites)
  - [Hardware](#hardware)
  - [rootfs on SSD via USB](#rootfs-on-ssd-via-usb)
  - [Packages](#packages)
  - [The CO₂ meter/monitor](#the-co2-metermonitor)
  - [OpenSSL](#openssl)
  - [InfluxDB](#influxdb)
  - [Grafana](#grafana)
  - [LCD](#lcd)
  - [Waveshare e-Paper display](#waveshare-e-paper-display)
- [Cross-compilation](#cross-compilation)
  - [debootstrap 🐥](#debootstrap)
- [TODOs](#todos)

<!-- /code_chunk_output -->

## Goals

- [x] 🖥 Monitor carbon dioxide (CO₂) levels
- [ ] 👀 Show (at least) current data on a connected display
- [x] 📝 Record data as timeseries
- [ ] 🚪 Make timeseries data accessible

### Maybe or Maybe not

- ☁️ export/sync to remote/"the cloud"
- ⚠️ alert people when value is too high
- 😈 make such alerts audible

## Visuals

### Dashboard example (Grafana)

![](./dashboard-full.png)

-----

## Prerequesites

### Hardware

- Raspberry Pi (model 3 or 4)
  - model 4 preferred, has definitely better performance around USB,
    plus you can bump up the RAM to 4 GB if you like
- SSD + SATA-to-USB adapter; disk size: whatever your wallet allows
- display, either LCD or e-paper (the latter I bought out of curiosity)
- the CO2 monitor (details see below), otherwise you would need to guess the data 😉
- 3D printer, so that you can build a nice and fitting case for the mess

If you in CNC milling or carpentry, replace the 3D printer with your tools.

### rootfs on SSD via USB

Follow this guide: <https://www.raspberrypi.org/forums/viewtopic.php?t=44177>

Why? SD cards are not very good for very frequent writes.
Since we want to run the system and also store the timeseries data on the Raspberry Pi, an SSD is highly recommended and preferred.

### Packages

Needed for later steps.

```sh
sudo apt-get update

# if you use the pi also as the compilation environment
sudo apt-get install libusb-1.0-0-dev libssl-dev

# if you compile somewhere else and only run the binary on the pi
sudo apt-get install libusb-1.0-0 libssl
```

Why?
- At one point some crate will ask for a native (open)ssl implementation, which requires to be compiled from sources.
  Might check in the future if I can still get to a pure Rust implementation.
- libusb is a requirement for the CO2 monitor, which has only USB connection for convenience.
  I tried to build with pure Rust hidapi/usb stuff, but didn't work.

### The CO₂ meter/monitor

For this project:

- **TFA Dostmann AirCO2ntrol Mini CO₂-Monitor** (what I've got)
- (aka) CO2Meter<!--d-->.com CO2Mini (model RAD-0301)
- or any clone (there are sometimes differently branded ones online)

Price differences are not that big, so pick any available one.
Sadly the costly part of that is the sensor already, therefore never expect a significant price drop anytime soon.

Product: <https://www.co2meter.com/products/co2mini-co2-indoor-air-quality-monitor>
Manual: <https://co2meters.com/Documentation/Manuals/Manual-RAD-0301.pdf>

For raspbian (debian):

* Install dependencies (co2mon/hidapi works only reliably with statically compiled libusb)

  ```sh
  sudo apt-get update
  sudo apt-get install libusb-1.0-0-dev
  ```

* Add udev rules file `/etc/udev/rules.d/60-co2mon.rules`

  ```txt
  ACTION=="add|change", SUBSYSTEMS=="usb", ATTRS{idVendor}=="04d9", ATTRS{idProduct}=="a052", MODE:="0666"
  ```

  You can verify if the vendor and product IDs match with your device via `lsusb`:

  ```txt
  Bus 001 Device 004: ID 04d9:a052 Holtek Semiconductor, Inc. USB-zyTemp
  […]
  ```

* Reload and trigger

  ```sh
  udevadm control --reload
  udevadm trigger
  ```

### OpenSSL

Some crates' dependencies come with openssl-sys as a depencency, so it needs to be able to compile it on the system.

In our case here it's because of `reqwest` (the underlying HTTP client for the influxdb client)

If not done yet:

```sh
sudo apt-get install libssl-dev
```

### InfluxDB

Guide: <https://pimylifeup.com/raspberry-pi-influxdb/>

```sh
wget -qO- https://repos.influxdata.com/influxdb.key | sudo apt-key add -
echo "deb https://repos.influxdata.com/debian buster stable" | sudo tee /etc/apt/sources.list.d/influxdb.list
sudo apt update
sudo apt install influxdb
sudo systemctl unmask influxdb
sudo systemctl enable influxdb
sudo systemctl start influxdb
```

### Grafana

This frontend is still much better than the influx chronograph.
The new `flux` stuff was not tested yet, maybe it's an option for the future.

In the end you can swap this with any dashboard solution, as long as it can easily ingest influxdb data.

Guide: <http://blog.centurio.net/2018/10/28/howto-install-influxdb-and-grafana-on-a-raspberry-pi-3/>
Download: <https://grafana.com/grafana/download/6.5.2?platform=arm>

```sh
sudo apt-get install -y adduser libfontconfig1
wget https://dl.grafana.com/oss/release/grafana_6.5.2_armhf.deb
sudo dpkg -i grafana_6.5.2_armhf.deb
sudo systemctl daemon-reload
sudo systemctl enable grafana-server
sudo systemctl start grafana-server

# http://<ip-of-grafana-machine>:3000 - admin/admin
# change password after first login!
```

### LCD

Shop any decent 7 inch display, ideally with high resolution.

The one I got: `GeeekPi 7 inch 1024x600 HDMI Screen LCD Display with Driver Board Monitor for Raspberry Pi`

If you work with a Raspberry Pi 4, keep in mind to have a _Micro HDMI to HDMI_ adapter.

No special software requirements, works like any usual computer display.

### Waveshare e-Paper display

Note: use only if LCD is not an option

* Enable SPI support: run `sudo raspi-config` and enable it
  (this will modify the `/boot/config.txt` for you)
  Check, if your user is also member of the group `spi`;
  if not run `sudo adduser <username> spi`

* Use `https://github.com/asaaki/epd-waveshare`, branch: `epd7in5_v2`, if you have also a **version 2** 7.5" display,
  otherwise the regular crate `epd-waveshare` will do fine.

-----

## Cross-compilation

Funnily enough the Raspberry Pi 3 and 4 are quite powerful for some Rust compilation.
So you can build the software right on device.

But maybe you do have a powerful machine and want to utilize it for builds and tests.
(As long as no real hardware/peripherals/sensors are required.)

If you want to read about my lost weekend: [Cross compilations fails](./CROSSCOMPILATIONFAILS.md)

But more importantly, what does work?

### debootstrap 🐥

Bootstrap an debian rootfs for armv7 (in distro lingo called `armhf`).
Install your native dependency development packages.
Install Rust.
Compile ahead.

The folder `cross/` includes some scripts to semi automate this process.

You want to consider a different location for the rootfs depending on your disks.
If you have mixed system like me with spinning HDDs and SSDs, put/move the rootfs to an SSD for some more performance.

**But most important note: To not run into the same qemu/ext4 issue, do not have it on an ext4 partition!**

Spare some space on a disk and format it with btrfs, XFS, ZFS, or some other Linux supported filesystem.

## TODOs

- [ ] Turn setup into a semi or fully automated script
  - [x] Cross compilation on host: _For now very semi, but good enough for my use-case._
- [ ] Build a case for the components
- [ ] Deploy at work
- [ ] Impress coworkers

-----

🚏
