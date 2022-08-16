# Easy Logging Library

### Examples

#### Logging to a file and console

```rust
mod lib;

use lib::Log;

fn main() {
    let mut logger: Log = Log::new();
    logger.output_to_file();
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.debug("Hello, world!");
}
```

#### Logging to just the console

```rust
mod lib;

use lib::Log;

fn main() {
    let logger: Log = Log::new();
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.debug("Hello, world!");
}
```