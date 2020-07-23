use std::fmt;
use std::iter;

use crate::coord::{Point, PointDebug};
use crate::create_id_type;

create_id_type!(ShapeId);

#[derive(PartialEq, Clone)]
pub(crate) struct Shape {
    points: Vec<Point>,
}

impl Shape {
    pub(super) fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub(super) fn add(&mut self, position: Point) {
        self.points.push(position);
    }

    pub(crate) fn iter_count(&self, count: usize) -> impl Iterator<Item = Point> + '_ {
        self.points
            .iter()
            .chain(
                iter::repeat(self.points.last().unwrap())
                    .take(count.saturating_sub(self.points.len())),
            )
            .copied()
    }

    #[cfg(test)]
    pub(in crate::shape) fn reversed(mut self) -> Self {
        self.points.reverse();
        self
    }
}

impl From<Vec<Point>> for Shape {
    fn from(value: Vec<Point>) -> Self {
        Self { points: value }
    }
}

impl IntoIterator for Shape {
    type Item = Point;
    type IntoIter = <Vec<Point> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Debug for Shape {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_list()
            .entries(
                self.points
                    .iter()
                    .map(|&position| PointDebug::new(position, 6)),
            )
            .finish()
    }
}

#[cfg(test)]
pub(crate) mod fixtures {
    macro_rules! shapes {
        ($($line:ident: {$($shape:ident => [$($lat:expr, $lon:expr);* $(;)?]),* $(,)?}),* $(,)?) => (
            $(
                pub(crate) mod $line {
                    use std::collections::HashMap;

                    use test_utils::map;
                    use crate::coord::project;
                    use crate::shape::{Shape, ShapeId};

                    $(
                        pub(crate) fn $shape() -> Shape {
                            Shape {
                                points: vec![$( project($lat, $lon) ),*],
                            }
                        }
                    )*

                    #[allow(dead_code)]
                    pub(crate) fn by_id() -> HashMap<ShapeId, Shape> {
                        map! {
                            $( stringify!($shape) => $shape() ),*
                        }
                    }
                }
            )*
        );
    }

    shapes! {
        s41: {
            circle => [
                52.549, 13.388; 52.503, 13.469; 52.475, 13.366; 52.501, 13.283; 52.549, 13.388;
            ],
        },
        tram_m10: {
            clara_jaschke_str_warschauer_str => [
                52.525, 13.366; 52.526, 13.370; 52.529, 13.377; 52.530, 13.382; 52.532, 13.388;
                52.536, 13.390; 52.538, 13.396; 52.540, 13.401; 52.541, 13.406; 52.541, 13.412;
                52.540, 13.420; 52.539, 13.424; 52.538, 13.428; 52.536, 13.434; 52.534, 13.437;
                52.532, 13.441; 52.528, 13.445; 52.527, 13.447; 52.522, 13.450; 52.519, 13.453;
                52.516, 13.454; 52.512, 13.452; 52.508, 13.450; 52.505, 13.448;
            ],
            warschauer_str_lueneburger_str => [
                52.505, 13.448; 52.508, 13.450; 52.509, 13.451; 52.512, 13.452; 52.516, 13.454;
                52.519, 13.453; 52.522, 13.450; 52.527, 13.447; 52.528, 13.445; 52.532, 13.441;
                52.534, 13.437; 52.536, 13.434; 52.538, 13.428; 52.539, 13.424; 52.540, 13.420;
                52.541, 13.412; 52.541, 13.406; 52.540, 13.401; 52.538, 13.396; 52.536, 13.390;
                52.532, 13.388; 52.530, 13.382; 52.529, 13.377; 52.526, 13.370; 52.524, 13.363;
                52.523, 13.362;
            ],
            clara_jaschke_str_landsberger_allee_petersburger_str => [
                52.525, 13.366; 52.526, 13.370; 52.529, 13.377; 52.530, 13.382; 52.532, 13.388;
                52.536, 13.390; 52.538, 13.396; 52.540, 13.401; 52.541, 13.406; 52.541, 13.412;
                52.540, 13.420; 52.539, 13.424; 52.538, 13.428; 52.536, 13.434; 52.534, 13.437;
                52.532, 13.441; 52.528, 13.445; 52.527, 13.447;
            ],
            landsberger_allee_petersburger_str_lueneburger_str => [
                52.527, 13.447; 52.528, 13.445; 52.532, 13.441; 52.534, 13.437; 52.536, 13.434;
                52.538, 13.428; 52.539, 13.424; 52.540, 13.420; 52.541, 13.412; 52.541, 13.406;
                52.540, 13.401; 52.538, 13.396; 52.536, 13.390; 52.532, 13.388; 52.530, 13.382;
                52.529, 13.377; 52.526, 13.370; 52.524, 13.363; 52.523, 13.362;
            ],
        },
        tram_12: {
            oranienburger_tor_am_kupfergraben => [
                52.525, 13.388; 52.524, 13.388; 52.521, 13.388; 52.520, 13.388; 52.519, 13.388; 52.519, 13.389; 52.519, 13.390;
                52.519, 13.391; 52.519, 13.392; 52.519, 13.396;
            ],
            am_kupfergraben_oranienburger_tor => [
                52.519, 13.396; 52.520, 13.396; 52.521, 13.395; 52.521, 13.394; 52.520, 13.393; 52.520, 13.391; 52.520, 13.390;
                52.519, 13.390; 52.519, 13.389; 52.520, 13.388; 52.521, 13.388; 52.522, 13.388; 52.524, 13.388; 52.525, 13.388;
            ],
        },
        bus_m41: {
            anhalter_bahnhof_hauptbahnhof => [
                52.505, 13.382; 52.506, 13.380; 52.507, 13.380; 52.507, 13.379; 52.508, 13.378; 52.509, 13.377; 52.510, 13.377;
                52.511, 13.377; 52.512, 13.377; 52.512, 13.376; 52.512, 13.374; 52.511, 13.372; 52.511, 13.371; 52.512, 13.371;
                52.513, 13.371; 52.514, 13.371; 52.516, 13.371; 52.518, 13.372; 52.519, 13.372; 52.520, 13.373; 52.521, 13.373;
                52.521, 13.372; 52.5257,13.368; 52.526, 13.368; 52.527, 13.368; 52.528, 13.368; 52.527, 13.369;
            ],
            hauptbahnhof_anhalter_bahnhof => [
                52.527, 13.369; 52.526, 13.369; 52.5262,13.368; 52.522, 13.372; 52.521, 13.372; 52.520, 13.372; 52.518, 13.371;
                52.516, 13.371; 52.514, 13.370; 52.513, 13.371; 52.512, 13.371; 52.511, 13.371; 52.511, 13.372; 52.511, 13.374;
                52.512, 13.374; 52.512, 13.376; 52.512, 13.377; 52.511, 13.377; 52.510, 13.377; 52.509, 13.377; 52.508, 13.377;
                52.508, 13.378; 52.507, 13.379; 52.506, 13.380; 52.505, 13.382;
            ],
        },
        bus_114: {
            wannsee_heckeshorn_wannsee => [
                52.422, 13.178; 52.421, 13.178; 52.421, 13.177; 52.421, 13.176; 52.420, 13.175; 52.420, 13.174; 52.421, 13.174;
                52.421, 13.173; 52.421, 13.172; 52.421, 13.171; 52.421, 13.170; 52.421, 13.169; 52.421, 13.168; 52.421, 13.167;
                52.421, 13.166; 52.421, 13.165; 52.422, 13.165; 52.422, 13.164; 52.423, 13.163; 52.423, 13.162; 52.424, 13.162;
                52.425, 13.161; 52.426, 13.161; 52.427, 13.162; 52.428, 13.162; 52.428, 13.163; 52.429, 13.164; 52.430, 13.164;
                52.430, 13.165; 52.431, 13.165; 52.432, 13.165; 52.433, 13.164; 52.432, 13.163; 52.432, 13.162; 52.431, 13.162;
                52.431, 13.161; 52.430, 13.161; 52.429, 13.160; 52.428, 13.160; 52.427, 13.160; 52.427, 13.159; 52.426, 13.160;
                52.424, 13.160; 52.421, 13.162; 52.420, 13.162; 52.420, 13.166; 52.420, 13.167; 52.421, 13.168; 52.421, 13.170;
                52.421, 13.171; 52.421, 13.172; 52.421, 13.173; 52.420, 13.174; 52.420, 13.175; 52.420, 13.176; 52.421, 13.176;
                52.421, 13.177; 52.421, 13.178; 52.422, 13.179; 52.422, 13.180; 52.422, 13.179; 52.422, 13.178;
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! shape {
        ($($lat:expr, $lon:expr);*) => (
            vec![$($crate::coord::project($lat, $lon)),*]
        );
        (blue) => (
            $crate::shape!(52.526, 13.369; 52.523, 13.378; 52.520, 13.387; 52.521, 13.394; 52.523, 13.402)
        );
        ($shape:ident reversed) => ({
            let mut shape = $crate::shape!($shape);
            shape.reverse();
            shape
        });
    }
}
