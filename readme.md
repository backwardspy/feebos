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

## building and running

`cargo kbuild` builds the kernel binary
`cargo kimage` builds the kernel and disk image
`cargo krun` builds the kernel and disk image, then launches it in qemu
`cargo ktest` builds the kernel and disk image, then runs the tests in a headless qemu
