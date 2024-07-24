# libisg

Library reading/writing the ISG 2.0-format file.

```rust
use std::fs;

use libisg;
use libisg::{Data, ISG};

let s = fs::read_to_string("file.isg").unwrap();
let isg = libisg::from_str(&s).unwrap();

match &isg.data {
    Data::Grid(data) => {
        for (nrow, row) in data.iter().enumerate() {
            for (ncol, value) in row.iter().enumerate() {
                // do something
            }
        }
    },
    Data::Sparse(data) => {
        for row in data {
            let (a, b, value) = row;
            // do something
        }
    }
}

// serialize to ISG-format
assert_eq!(s, isg.to_string());

// serialize/deserialize by serde
use serde_json;
 
let json = serde_json::to_string(&isg).unwrap();
let isg: ISG = serde_json::from_string(&json).unwrap();
```

## Licence

MIT or Apache-2.0

## Reference

Specification: https://www.isgeoid.polimi.it/Geoid/format_specs.html