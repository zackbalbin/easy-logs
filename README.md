# Simple Logging Library

### Example

```rust
mod lib;

use lib::Log;

fn main() {
    let logger: Log = Log::new(true);
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.debug("Hello, world!");
}
```