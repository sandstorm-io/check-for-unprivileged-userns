# Using Rust to make a small user namespaces test binary

See https://github.com/kmcallister/tiny-rust-demo for build instructions, etc.

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
