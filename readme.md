# ![feebas facing right](assets/feebas_right.png) feebos ![feebas facing left](assets/feebas_left.png)

like the pokemon, but it's an OS... or something. when i was writing a
hexadecimal print routine in the original bootloader, i tested it by trying to
print 0xBEEF. turns out i got it backwards and the output was reversed.

## build environment

you will need nightly rust with `rust-src` and `llvm-tools-preview` components.

example setup:

```shell
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview
```

## building a bootable feebos image

_note: running the build script requires python 3. you can build without running
the script, you'll just need to check the script and run the same commands in
the same order._

```shell
./build          # for a debug image
./build release  # for a release image
```

## running feebos in qemu

once you've got a bootable image, run it in qemu like so:

```shell
qemu-system-x86_64 -drive format=raw,file=target/x86_64-feebos/debug/boot-bios-feebos.img
```

replace `debug` with `release` if you're running a release build.
