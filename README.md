# Show file processing time from PHPStan log

> This is only a [rust](https://www.rust-lang.org) learning project. Comments and corrections are highly encouraged.

See [original PHP implementation](https://gist.github.com/ruudk/41897eb59ff497b271fc9fa3c7d5fb27) for context.

## Usage

1. Run PHPStan, saving the output to a log: `vendor/bin/phpstan analyse --memory-limit=-1 --debug -vvv | tee phpstan.log`.
2. Display timing info with `target/release/phpstan-timer path-to/phpstan.log`.
3. See `sample/phpstan.log` for sample input.

## Build
_You need a working rust installation._
Run `cargo build --release` to make an optimised binary.
