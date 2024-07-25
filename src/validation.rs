use crate::error::ValidationError;
use crate::parse::HeaderField;
use crate::{Coord, CoordType, CoordUnits, Data, DataBounds, DataFormat, ISG};

impl ISG {
    /// Return `true` if data if well-formatted
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Validate adaptation to ISG-format
    ///
    /// This checks:
    /// - `data_bounds` by `data_format` and `coord_type`
    /// - data format of `data_bounds` and data by `coord_units`
    /// - data length by `nrows` and `ncols`
    pub fn validate(&self) -> Result<(), ValidationError> {
        match (&self.header.data_format, &self.header.coord_type) {
            (DataFormat::Grid, CoordType::Geodetic) => {
                if !matches!(&self.header.data_bounds, DataBounds::GridGeodetic { .. }) {
                    return Err(ValidationError::data_bounds(
                        self.header.data_format.clone(),
                        self.header.coord_type.clone(),
                    ));
                }
            }
            (DataFormat::Grid, CoordType::Projected) => {
                if !matches!(&self.header.data_bounds, DataBounds::GridProjected { .. }) {
                    return Err(ValidationError::data_bounds(
                        self.header.data_format.clone(),
                        self.header.coord_type.clone(),
                    ));
                }
            }
            (DataFormat::Sparse, CoordType::Geodetic) => {
                if !matches!(&self.header.data_bounds, DataBounds::SparseGeodetic { .. }) {
                    return Err(ValidationError::data_bounds(
                        self.header.data_format.clone(),
                        self.header.coord_type.clone(),
                    ));
                }
            }
            (DataFormat::Sparse, CoordType::Projected) => {
                if !matches!(&self.header.data_bounds, DataBounds::SparseProjected { .. }) {
                    return Err(ValidationError::data_bounds(
                        self.header.data_format.clone(),
                        self.header.coord_type.clone(),
                    ));
                }
            }
        };

        let is_valid_coord = match &self.header.coord_units {
            CoordUnits::DMS => |a: &Coord| matches!(a, Coord::DMS { .. }),
            CoordUnits::Deg | CoordUnits::Meters | CoordUnits::Feet => {
                |a: &Coord| matches!(a, Coord::Dec { .. })
            }
        };

        match &self.header.data_bounds {
            DataBounds::GridGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
                delta_lat,
                delta_lon,
            } => {
                if !is_valid_coord(lat_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::LatMin));
                } else if !is_valid_coord(lat_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::LatMax));
                } else if !is_valid_coord(lon_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::LonMin));
                } else if !is_valid_coord(lon_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::LonMax));
                } else if !is_valid_coord(delta_lat) {
                    return Err(ValidationError::coord_units_header(HeaderField::DeltaLat));
                } else if !is_valid_coord(delta_lon) {
                    return Err(ValidationError::coord_units_header(HeaderField::DeltaLon));
                }
            }
            DataBounds::GridProjected {
                north_min,
                north_max,
                east_min,
                east_max,
                delta_north,
                delta_east,
            } => {
                if !is_valid_coord(north_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::NorthMin));
                } else if !is_valid_coord(north_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::NorthMax));
                } else if !is_valid_coord(east_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::EastMin));
                } else if !is_valid_coord(east_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::EastMax));
                } else if !is_valid_coord(delta_north) {
                    return Err(ValidationError::coord_units_header(HeaderField::DeltaNorth));
                } else if !is_valid_coord(delta_east) {
                    return Err(ValidationError::coord_units_header(HeaderField::DeltaEast));
                }
            }
            DataBounds::SparseGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
            } => {
                if !is_valid_coord(lat_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::LatMin));
                } else if !is_valid_coord(lat_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::LatMax));
                } else if !is_valid_coord(lon_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::LonMin));
                } else if !is_valid_coord(lon_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::LonMax));
                }
            }
            DataBounds::SparseProjected {
                north_min,
                north_max,
                east_min,
                east_max,
            } => {
                if !is_valid_coord(north_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::NorthMin));
                } else if !is_valid_coord(north_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::NorthMax));
                } else if !is_valid_coord(east_min) {
                    return Err(ValidationError::coord_units_header(HeaderField::EastMin));
                } else if !is_valid_coord(east_max) {
                    return Err(ValidationError::coord_units_header(HeaderField::EastMax));
                }
            }
        };

        match &self.data {
            Data::Grid(data) => {
                if data.len() != self.header.nrows {
                    return Err(ValidationError::nrows(self.header.nrows, data.len()));
                }

                for row in data {
                    if row.len() != self.header.ncols {
                        return Err(ValidationError::ncols(self.header.ncols, Some(row.len())));
                    }
                }
            }
            Data::Sparse(data) => {
                if data.len() != self.header.nrows {
                    return Err(ValidationError::nrows(self.header.nrows, data.len()));
                }

                if 3 != self.header.ncols {
                    return Err(ValidationError::ncols(self.header.ncols, None));
                }

                for (lineno, row) in data.iter().enumerate() {
                    if !is_valid_coord(&row.0) {
                        return Err(ValidationError::coord_units_data(lineno + 1, 1));
                    } else if !is_valid_coord(&row.1) {
                        return Err(ValidationError::coord_units_data(lineno + 1, 2));
                    }
                }
            }
        };

        Ok(())
    }
}
