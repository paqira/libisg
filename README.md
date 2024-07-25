# libisg

Library reading/writing the ISG 2.0-format.

```rust
use std::fs;

use libisg;
use libisg::{Data, DataBounds, ISG};

let s = fs::read_to_string("file.isg").unwrap();

let isg = libisg::from_str(&s).unwrap();


let (a_max, b_max, delta_a, delta_b) = match isg.header.data_bounds {
    DataBounds::GridGeodetic { lat_max, lon_max, delta_lat, delta_lon, .. } => {
        (lat_max, lon_max, delta_lat, delta_lon)
    },
    _ => unimplemented!("`file.isg` is grid geodetic"),
};

match &isg.data {
    Data::Grid(data) => {
        for (nrow, row) in data.iter().enumerate() {
            for (ncol, value) in row.iter().enumerate() {
                let a = a_max - delta_a * nrow;
                let b = b_max + delta_b * ncol;
                // do something
            }
        }
    }
    Data::Sparse(data) => {
        for row in data {
            let (a, b, value) = row;
            // do something
        }
    }
}
```

Features:

- Support serialization/deserialization of ISG format
- Support `serde` (feature `serde` required)

## Licence

MIT or Apache-2.0

## Reference

Specification: https://www.isgeoid.polimi.it/Geoid/format_specs.html