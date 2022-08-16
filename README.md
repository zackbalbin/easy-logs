# Easy Logging Library

A simple and lightweight rust logging library.

### Examples

#### Logging to a file and console

```rust
use lib::Logger;

fn main() {
    let mut logger: Logger = Logger::new();
    logger.output_to_file(None);

    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.debug("Hello, world!");
}
```

#### Logging to just the console

```rust
use lib::Logger;

fn main() {
    let logger: Logger = Logger::new();
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.debug("Hello, world!");
}
```