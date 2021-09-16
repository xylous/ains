# ains - Arch INStaller

ains is a simple arch installer. It's not interactive; you feed it a YAML file
and then it does everything for you.

Instead of having to use imperative scripts that are harder to change or using
interactive scripts/programs that require you to input your preferences on every
install, you can now write a simple YAML file, use `ains`, and you'll be up and
running quickly.

## Getting started

### Requirements

- an active internet connection
- cargo or rust toolchain

### Installation

Not available on `crates.io`.

#### Manual

You can install ains by downloading this repository and building with `cargo`,
e.g.:

```
git clone https://github.com/xylous/ains.git ains
cd ains
cargo build
```

And either move onto `$PATH`, either run directly, for example `$ ./ains <...>`

### Usage

```
$ ains <file>.yaml
```

***WARNING***: don't run anywhere except in the arch install shell! This program
*will* overwrite your system!

## Roadmap

- [ ] install and set default shell
- [ ] set root and user password
- [ ] partitions
    - [ ] mount point
    - [ ] adjustable size
    - [ ] support swap partitions
- [ ] install and configure bootloader/boot manager
    - [ ] from EFI directly (efistub)
    - [ ] grub
- [ ] different/multiple kernels
- [ ] generate template YAML file on the spot

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
