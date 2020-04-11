# proxycat

`proxycat` makes it easy to transparently proxy a specific application's TCP
traffic.

`proxycat` is a largely a wrapper around `iptables` and has to be run on an
Android device with root privileges. `proxycat` does not take into
consideration existing iptables rules on the device and might conflict with or
clobber existing rules. Use with caution.

## Usage

```
➜ ./proxycat
proxycat 0.1.0
Terry Chia <terrycwk1994@gmail.com>


USAGE:
    proxycat [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Add proxy rule.
    clean    Remove iptable NAT rules.
    help     Prints this message or the help of the given subcommand(s)
```

### `proxycat add`

This subcommand is used to add a new `iptables` rule and requires the package
name and proxy address as arguments.

```
➜ ./proxycat add --help
proxycat-add
Add proxy rule.

USAGE:
    proxycat add <PACKAGE> <PROXY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PACKAGE>    Android app to proxy.
    <PROXY>      Proxy address to use.
```

### `proxycat clean`

The `clean` subcommand removes all `nat` rules from the device.

## How It Works

`iptables` rules can be configured to apply to packets created by a specific
user with the `--uid-owner` option.

This can be used to create `iptables` rules that apply to a specific
application as every Android app is assigned a unique UID at install time. This
UID is stored in the `/data/system/packages.list` file which is parsed by
`proxycat`.

`nat` rules are then added to transparently proxy traffic to the specified
proxy address. To view the inserted rules, run `iptables -t nat -L` on device.

## Building

`proxycat` can be compiled into a static binary with the following command:

```
➜ cargo build --target x86_64-unknown-linux-musl --release
```

If a binary for another architecture is required, `proxycat` can be
cross-compiled using [cross][cross].

```
➜ cross build --target arm-unknown-linux-musleabi --release
```

[cross]: https://github.com/rust-embedded/cross
