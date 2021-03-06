use std::rc::Rc;

use serde_derive::{Deserialize, Serialize};

use crate::node::Node;
use crate::schedule::Schedule;
use crate::train::Train;
use simulation::line::Kind;
use simulation::Color;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    name: String,
    color: Color,
    kind: Kind,
    nodes: Vec<Node>,
    trains: Vec<Train>,
}

impl Line {
    pub fn new(
        name: String,
        color: Color,
        kind: Kind,
        nodes: Vec<Node>,
        trains: Vec<Train>,
    ) -> Line {
        Line {
            name,
            color,
            kind,
            nodes,
            trains,
        }
    }

    pub(crate) fn add_to_station_infos(&self, station_infos: &mut Vec<Vec<Kind>>) {
        let station_ids = self.nodes.iter().filter_map(Node::station);
        for station_id in station_ids {
            let station_info = &mut station_infos[station_id];
            if !station_info.contains(&self.kind) {
                station_info.push(self.kind);
            }
        }
    }

    pub fn load(
        self,
        stations: &[Rc<simulation::Station>],
        schedules: &[Schedule],
    ) -> simulation::Line {
        let kind = self.kind;
        let nodes = self
            .nodes
            .into_iter()
            .map(|node| node.load(&stations))
            .collect::<Vec<_>>();
        let trains = self
            .trains
            .into_iter()
            .map(|train| train.load(kind, &nodes, schedules))
            .collect();

        simulation::Line::new(self.name, self.color, kind, nodes, trains)
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use std::ops::Index;

    use super::*;
    use crate::fixtures::{nodes, trains};
    use test_utils::time;

    macro_rules! lines {
        (@trains $line:ident, $route:ident, [$( $( $(:)? $time:literal )* ),* $(,)?], $schedule_ids:expr) => {
            $( trains::$line::$route(time!($($time),*), $schedule_ids) ),*
        };
        ($($line:ident: $name:literal, $kind:ident, $upstream:ident, $upstream_times:tt, $downstream:ident, $downstream_times:tt);* $(;)?) => {
            $(
                pub fn $line<'a>(
                    station_ids: &impl Index<&'a str, Output = usize>,
                    schedule_ids: &impl Index<&'a str, Output = usize>,
                ) -> Line {
                    Line {
                        name: $name.to_string(),
                        color: Kind::$kind.color(),
                        kind: Kind::$kind,
                        nodes: nodes::$line(station_ids),
                        trains: vec![
                            lines!(@trains $line, $upstream, $upstream_times, schedule_ids),
                            lines!(@trains $line, $downstream, $downstream_times, schedule_ids),
                        ],
                    }
                }
            )*
        };
    }

    lines! {
        s3:                 "S3",           SuburbanRailway,
            hackescher_markt_bellevue, [7:24:54],
            bellevue_hackescher_markt, [7:12:24];
        u6:                 "U6",           UrbanRailway,
            naturkundemuseum_franzoesische_str, [5:55:40],
            franzoesische_str_naturkundemuseum, [5:29:40];
        tram_12:            "12",           Tram,
            oranienburger_tor_am_kupfergraben, [9:01:40],
            am_kupfergraben_oranienburger_tor, [8:33:40];
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::fixtures::lines;
    use crate::fixtures_with_ids;
    use test_utils::map;

    #[test]
    fn test_add_tram_to_station_infos() {
        let station_ids: HashMap<&str, usize> = map! {
            "oranienburger_tor" => 1,
            "friedrichstr" => 2,
            "universitaetsstr" => 3,
            "am_kupfergraben" => 4,
            "georgenstr_am_kupfergraben" => 5,
        };
        let schedule_ids: HashMap<&str, usize> = map! {
            "oranienburger_tor_am_kupfergraben" => 0,
            "am_kupfergraben_oranienburger_tor" => 1,
        };
        let mut station_infos = vec![Vec::new(); 7];
        let line = lines::tram_12(&station_ids, &schedule_ids);
        line.add_to_station_infos(&mut station_infos);
        let station_ids = station_ids.values().copied().collect::<Vec<_>>();
        for (station_id, station_info) in station_infos.into_iter().enumerate() {
            let expect_contained = station_ids.contains(&station_id);
            assert_eq!(station_info.contains(&Kind::Tram), expect_contained);
        }
    }

    #[test]
    fn test_load() {
        let (stations, station_ids) = fixtures_with_ids!(simulation::stations::{
            oranienburger_tor,
            friedrichstr,
            universitaetsstr,
            am_kupfergraben,
            georgenstr_am_kupfergraben,
        } with Rc);
        let (schedules, schedule_ids) = fixtures_with_ids!(schedules::{
           oranienburger_tor_am_kupfergraben,
           am_kupfergraben_oranienburger_tor,
        });
        let line = lines::tram_12(&station_ids, &schedule_ids);
        assert_eq!(
            line.load(&stations, &schedules),
            simulation::fixtures::lines::tram_12()
        );
    }
}
