use std::collections::HashMap;

use serde_derive::Deserialize;

use crate::coord::project;
use super::{Shape, ShapeId};

#[derive(Debug, Deserialize)]
pub(super) struct ShapeRecord {
    shape_id: ShapeId,
    shape_pt_lat: f64,
    shape_pt_lon: f64,
}

impl ShapeRecord {
    pub(super) fn import(self, shapes: &mut HashMap<ShapeId, Shape>) {
        let waypoint = project(self.shape_pt_lat, self.shape_pt_lon);
        shapes.entry(self.shape_id)
            .or_insert_with(Vec::new)
            .push(waypoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::map;

    #[test]
    fn test_import() {
        let mut shapes = HashMap::new();
        let record = ShapeRecord {
            shape_id: "1".into(),
            shape_pt_lat: 52.526,
            shape_pt_lon: 13.369,
        };
        record.import(&mut shapes);
        assert_eq!(shapes, map! {
            "1" => vec![project(52.526, 13.369)],
        });
    }
}