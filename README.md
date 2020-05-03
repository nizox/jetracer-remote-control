# JetRacer Remote Control

## Building from source

First, install the dependencies:
```shell
dnf install glib2-devel gstreamer1-devel
```

## Cross-compilation for Jetson Nano

First, get a aarch64 toolchain:
```shell
curl -L -o /dev/null https://developer.arm.com/-/media/Files/downloads/gnu-a/9.2-2019.12/binrel/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu.tar.xz
tar -xf gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu.tar.xz
```

TODO add missing libraries to the toolchain (glib and gstreamer)

Then, setup the cargo target:
```shell
rustup target add aarch64-unknown-linux-gnu
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=${PWD}/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/aarch64-none-linux-gnu/bin/ld
export CC_aarch64_unknown_linux_gnu=${PWD}/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/bin/aarch64-none-linux-gnu-gcc
```

Finally, build the binary for the Jetson Nano:
```shell
cargo build --target aarch64-unknown-linux-gnu
```