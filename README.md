# Using Rust to make a small user namespaces test binary

This repo contains source code to a program that, in less than 200 bytes
of compiled binary, figures out if your system supports Linux's user
namespaces feature. I created it for the Sandstorm installer so that I can
embed a compiled binary in the install shell script (which is admittedly
a strange thing to do) because I could find no better way to check for
that kernel feature.

This repo contains the Rust source code for that binary, as well as a hand-written
assembly version.

At the time of writing, this compiles down to a program 157 bytes long. That
number might be out of date, so feel free to re-compile it yourself.

To build the Rust version, we use a custom toolchain. Keegan McAllister developed
that toolchain for https://github.com/kmcallister/tiny-rust-demo ; this project merely
re-uses it.

Massive thanks to @kmc and @bstrie for helping me create this silly thing.

## The code in the Sandstorm install script

To rebuild it, run:

```
./build.sh
```

The source code is in:

```
tinyrust.rs
```



## Alternate implementation in assembly

See

```
tinyasm.s
```

and

```
./build-asm.sh
```

The assembly strategy shaves 4 bytes off the result, but the installer
still uses the Rust version for now.
