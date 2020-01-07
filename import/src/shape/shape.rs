use na::Point2;

use crate::create_id_type;

create_id_type!(ShapeId);

pub(crate) type Shape = Vec<Point2<f32>>;

pub(crate) fn transform(lat: f32, lon: f32) -> Point2<f32> {
    Point2::new(lon, 2.0 * lat)
}

#[cfg(test)]
pub(crate) mod fixtures {
    macro_rules! shapes {
        ($($line:ident: {$($shape:ident => [$($lat:expr, $lon:expr);* $(;)?]),* $(,)?}),* $(,)?) => (
            $(
                pub(crate) mod $line {
                    use std::collections::HashMap;

                    use crate::map;
                    use crate::shape::{Shape, ShapeId, transform};

                    $(
                        pub(crate) fn $shape() -> Shape {
                            vec![$(transform($lat, $lon)),*]
                        }
                    )*

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
    }
}

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! shape {
        ($($lat:expr, $lon:expr);*) => (
            vec![$($crate::shape::transform($lat, $lon)),*]
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
