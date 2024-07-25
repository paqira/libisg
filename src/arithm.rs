// TODO: better impl

use core::ops::{Add, Mul, Neg};
use std::ops::Sub;

use crate::Coord;

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Neg::neg(&self)
    }
}

impl Neg for &Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        match self {
            Coord::DMS {
                degree,
                minutes,
                second,
            } => Coord::DMS {
                degree: -degree,
                minutes: *minutes,
                second: *second,
            },
            Coord::Dec(f) => Coord::Dec(-f),
        }
    }
}

macro_rules! impl_mul {
    ($type:tt) => {
        impl Mul<$type> for Coord {
            type Output = Coord;

            fn mul(self, rhs: $type) -> Self::Output {
                Mul::mul(&self, &rhs)
            }
        }

        impl Mul<$type> for &Coord {
            type Output = Coord;

            fn mul(self, rhs: $type) -> Self::Output {
                Mul::mul(self, &rhs)
            }
        }

        impl Mul<&$type> for Coord {
            type Output = Coord;

            fn mul(self, rhs: &$type) -> Self::Output {
                Mul::mul(&self, rhs)
            }
        }

        impl Mul<&$type> for &Coord {
            type Output = Coord;

            fn mul(self, rhs: &$type) -> Self::Output {
                if *rhs == 0 {
                    return match self {
                        Coord::DMS { .. } => Coord::DMS {
                            degree: 0,
                            minutes: 0,
                            second: 0,
                        },
                        Coord::Dec(..) => Coord::Dec(0.0),
                    };
                }

                match self {
                    Coord::DMS {
                        degree,
                        minutes,
                        second,
                    } => {
                        let second = *second as u64;
                        let minutes = *minutes as u64;
                        let degree = *degree as i64;
                        let rhs = *rhs as u64;

                        let temp = second * rhs;
                        let (second, carry) = (temp % 60, temp / 60);

                        let temp = minutes * rhs + carry;
                        let (minutes, carry) = (temp % 60, temp / 60);

                        let degree = if !degree.is_negative() {
                            degree * rhs as i64 + carry as i64
                        } else {
                            degree * rhs as i64 - carry as i64
                        };

                        Coord::DMS {
                            degree: degree as i16,
                            minutes: minutes as u8,
                            second: second as u8,
                        }
                    }
                    Coord::Dec(coord) => Coord::Dec(coord * *rhs as f64),
                }
            }
        }
    };
}

impl_mul!(u8);
impl_mul!(u16);
impl_mul!(u32);
impl_mul!(u64);
impl_mul!(usize);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Add::add(&self, &rhs)
    }
}

impl Add<&Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Add::add(&self, rhs)
    }
}

impl Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Add::add(self, &rhs)
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        match (self, rhs) {
            (
                Coord::DMS {
                    degree: a_deg,
                    minutes: a_min,
                    second: a_sec,
                },
                Coord::DMS {
                    degree: b_deg,
                    minutes: b_min,
                    second: b_sec,
                },
            ) => {
                let a_sec = *a_sec as u64;
                let b_sec = *b_sec as u64;
                let a_min = *a_min as u64;
                let b_min = *b_min as u64;
                let a_deg = *a_deg as i64;
                let b_deg = *b_deg as i64;

                let temp = a_sec + b_sec;
                let (second, carry) = if 60 <= temp {
                    (temp - 60, 1)
                } else {
                    (temp, 0)
                };

                let temp = a_min + b_min + carry;
                let (minutes, carry) = if 60 <= temp {
                    (temp - 60, 1)
                } else {
                    (temp, 0)
                };

                let degree = a_deg + b_deg + carry as i64;
                Coord::DMS {
                    degree: degree as i16,
                    minutes: minutes as u8,
                    second: second as u8,
                }
            }
            (Coord::Dec(a), Coord::Dec(b)) => Coord::Dec(a + b),
            _ => unimplemented!("not supported ops: `Coord::DMS` + `Coord::Dec`"),
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Sub::sub(&self, &rhs)
    }
}

impl Sub<&Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        Sub::sub(&self, rhs)
    }
}

impl Sub<Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Sub::sub(self, &rhs)
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        match (self, rhs) {
            (
                Coord::DMS {
                    degree: a_deg,
                    minutes: a_min,
                    second: a_sec,
                },
                Coord::DMS {
                    degree: b_deg,
                    minutes: b_min,
                    second: b_sec,
                },
            ) => {
                let a_sec = *a_sec as i64;
                let b_sec = *b_sec as i64;
                let a_min = *a_min as i64;
                let b_min = *b_min as i64;
                let a_deg = *a_deg as i64;
                let b_deg = *b_deg as i64;

                let (second, carry) = if a_sec >= b_sec {
                    (a_sec - b_sec, 0)
                } else {
                    (60 + a_sec - b_sec, 1)
                };

                let (minutes, carry) = if a_min >= b_min + carry {
                    (a_min - b_min - carry, 0)
                } else {
                    (60 + a_min - b_min - carry, 1)
                };

                let degree = a_deg - b_deg - carry;
                Coord::DMS {
                    degree: degree as i16,
                    minutes: minutes as u8,
                    second: second as u8,
                }
            }
            (Coord::Dec(a), Coord::Dec(b)) => Coord::Dec(a - b),
            _ => unimplemented!(
                "not supported ops: `Coord::DMS` - `Coord::Dec` or `Coord::Dec` - `Coord::DMS`"
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{from_str, Data, DataBounds};

    use super::*;

    #[test]
    fn test() {
        let s = fs::read_to_string("rsc/isg/example.1.isg").unwrap();
        let isg = from_str(&s).unwrap();

        let (a_max, b_max, delta_a, delta_b) = match isg.header.data_bounds {
            DataBounds::GridGeodetic {
                lat_max,
                lon_max,
                delta_lat,
                delta_lon,
                ..
            } => (lat_max, lon_max, delta_lat, delta_lon),
            DataBounds::GridProjected {
                north_max,
                east_max,
                delta_north,
                delta_east,
                ..
            } => (north_max, east_max, delta_north, delta_east),
            DataBounds::SparseGeodetic {
                lat_max, lon_max, ..
            } => (lat_max, lon_max, Coord::Dec(0.0), Coord::Dec(0.0)),
            DataBounds::SparseProjected {
                north_max,
                east_max,
                ..
            } => (north_max, east_max, Coord::Dec(0.0), Coord::Dec(0.0)),
        };

        let pos: Vec<Vec<_>> = match &isg.data {
            Data::Grid(data) => data
                .iter()
                .enumerate()
                .map(|(nrow, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(ncol, _)| (a_max - delta_a * nrow, b_max - delta_b * ncol))
                        .collect()
                })
                .collect(),
            Data::Sparse(_) => {
                panic!()
            }
        };

        assert_eq!(
            pos,
            vec![
                vec![
                    (Coord::with_dms(41, 10, 0), Coord::with_dms(121, 50, 0)),
                    (Coord::with_dms(41, 10, 0), Coord::with_dms(121, 30, 0)),
                    (Coord::with_dms(41, 10, 0), Coord::with_dms(121, 10, 0)),
                    (Coord::with_dms(41, 10, 0), Coord::with_dms(120, 50, 0)),
                    (Coord::with_dms(41, 10, 0), Coord::with_dms(120, 30, 0)),
                    (Coord::with_dms(41, 10, 0), Coord::with_dms(120, 10, 0)),
                ],
                vec![
                    (Coord::with_dms(40, 50, 0), Coord::with_dms(121, 50, 0)),
                    (Coord::with_dms(40, 50, 0), Coord::with_dms(121, 30, 0)),
                    (Coord::with_dms(40, 50, 0), Coord::with_dms(121, 10, 0)),
                    (Coord::with_dms(40, 50, 0), Coord::with_dms(120, 50, 0)),
                    (Coord::with_dms(40, 50, 0), Coord::with_dms(120, 30, 0)),
                    (Coord::with_dms(40, 50, 0), Coord::with_dms(120, 10, 0)),
                ],
                vec![
                    (Coord::with_dms(40, 30, 0), Coord::with_dms(121, 50, 0)),
                    (Coord::with_dms(40, 30, 0), Coord::with_dms(121, 30, 0)),
                    (Coord::with_dms(40, 30, 0), Coord::with_dms(121, 10, 0)),
                    (Coord::with_dms(40, 30, 0), Coord::with_dms(120, 50, 0)),
                    (Coord::with_dms(40, 30, 0), Coord::with_dms(120, 30, 0)),
                    (Coord::with_dms(40, 30, 0), Coord::with_dms(120, 10, 0)),
                ],
                vec![
                    (Coord::with_dms(40, 10, 0), Coord::with_dms(121, 50, 0)),
                    (Coord::with_dms(40, 10, 0), Coord::with_dms(121, 30, 0)),
                    (Coord::with_dms(40, 10, 0), Coord::with_dms(121, 10, 0)),
                    (Coord::with_dms(40, 10, 0), Coord::with_dms(120, 50, 0)),
                    (Coord::with_dms(40, 10, 0), Coord::with_dms(120, 30, 0)),
                    (Coord::with_dms(40, 10, 0), Coord::with_dms(120, 10, 0)),
                ],
            ]
        )
    }
}
