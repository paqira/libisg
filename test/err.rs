use libisg::from_str;

#[test]
fn missing_start_of_header() {
    let s = r##"model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "missing line starts with `begin_of_head`"
    );
}

#[test]
fn missing_end_of_header() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0"##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "missing line starts with `end_of_head`"
    );
}

#[test]
fn missing_separator() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year      2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(a.unwrap_err().to_string(), "missing separator (line: 3)");
}

#[test]
fn invalid_header_key() {
    let s = r##"begin_of_head ================================================
 X:EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "unexpected header key: `X` (line: 2, column: 1 to 2)"
    );
}

#[test]
fn missing_header_key() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(a.unwrap_err().to_string(), "missing header key: `nrows`");
}

#[test]
fn duplicated_header_key() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model name     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "duplicated header key: `model name` (line: 3, column: 0 to 10)"
    );
}

#[test]
fn invalid_header_value() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     :X
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "unexpected value: `X` on `model type` (line: 4, column: 16 to 17)"
    );
}

#[test]
fn invalid_data_bounds() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
east max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head =================================================="##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "unexpected data bounds (lat max etc.)"
    );
}

#[test]
fn invalid_data() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : grid
data ordering  : N-to-S, W-to-E
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : dms
map projection : ---
EPSG code      : 7912
lat min        =   39°50'00"
lat max        =   41°10'00"
lon min        =  119°50'00"
lon max        =  121°50'00"
delta lat      =    0°20'00"
delta lon      =    0°20'00"
nrows          =           4
ncols          =           6
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head ==================================================
a
"##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "unexpected data: `a` (line: 29, column: 0 to 1)"
    );
}

#[test]
fn missing_data() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : sparse
data ordering  : lat, lon, N
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : deg
map projection : ---
EPSG code      : 7912
lat min        =   40.000000
lat max        =   41.000000
lon min        =  120.000000
lon max        =  121.666667
delta lat      = ---
delta lon      = ---
nrows          =          20
ncols          =           3
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head ==================================================
  40.000000  120.000000    30.1234
  40.000000
  40.000000  120.666667    32.3456
"##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "missing second column data (line: 30)"
    );
}

#[test]
fn invalid_sparse_data() {
    let s = r##"begin_of_head ================================================
model name     : EXAMPLE
model year     : 2020
model type     : gravimetric
data type      : geoid
data units     : meters
data format    : sparse
data ordering  : lat, lon, N
ref ellipsoid  : GRS80
ref frame      : ITRF2014
height datum   : ---
tide system    : mean-tide
coord type     : geodetic
coord units    : deg
map projection : ---
EPSG code      : 7912
lat min        =   40.000000
lat max        =   41.000000
lon min        =  120.000000
lon max        =  121.666667
delta lat      = ---
delta lon      = ---
nrows          =          20
ncols          =           3
nodata         =  -9999.0000
creation date  =  31/05/2020
ISG format     =         2.0
end_of_head ==================================================
  40.000000  120.000000    30.1234
  40.000000  120.000000    30.1234    30.1234
  40.000000  120.666667    32.3456
"##;
    let a = from_str(s);
    assert_eq!(
        a.unwrap_err().to_string(),
        "unexpected sparse data (line: 30)"
    );
}
