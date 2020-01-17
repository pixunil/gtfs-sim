use std::collections::HashMap;

use simulation::Color;
use simulation::line::Kind;
use crate::agency::AgencyId;
use crate::trip::Route;
use super::Line;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct IncompleteLine {
    agency_id: AgencyId,
    name: String,
    color: Option<Color>,
    kind: Kind,
}

impl IncompleteLine {
    pub(super) fn new(agency_id: AgencyId, name: String, kind: Kind) -> Self {
        Self {
            agency_id,
            name,
            color: None,
            kind,
        }
    }

    pub(super) fn add_color_when_applicable(&mut self, colors: &HashMap<String, Color>) {
        match self.kind {
            Kind::Railway | Kind::SuburbanRailway | Kind::UrbanRailway => {
                self.color = colors.get(&self.name).cloned();
            },
            _ => {},
        }
    }

    pub(super) fn finish(self, routes: Vec<Route>, lines: &mut HashMap<AgencyId, Vec<Line>>) {
        let line = Line::new(self.name, self.color.unwrap_or(self.kind.color()), self.kind, routes);
        lines.entry(self.agency_id)
            .or_insert_with(Vec::new)
            .push(line);
    }
}

#[cfg(test)]
pub(super) mod fixtures {
    use super::*;

    macro_rules! incomplete_lines {
        ($($line:ident: $agency:ident, $name:expr, $kind:ident);* $(;)?) => (
            $(
                pub(in crate::line) fn $line() -> IncompleteLine {
                    IncompleteLine::new(stringify!($agency).into(), $name.to_string(), Kind::$kind)
                }
            )*
        )
    }

    incomplete_lines! {
        blue:               pubtransport,   "Blue Line",    SuburbanRailway;
        blue_replacement:   pubtransport,   "Blue Line",    Bus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::map;
    use crate::line::fixtures::*;

    fn colors() -> HashMap<String, Color> {
        map! {
            "Blue Line" => Color::new(0, 0, 255),
        }
    }

    #[test]
    fn test_add_color_to_applicable() {
        let mut line = incomplete_lines::blue();
        line.add_color_when_applicable(&colors());
        assert_eq!(line.color, Some(Color::new(0, 0, 255)));
    }

    #[test]
    fn test_add_color_to_unapplicable() {
        let mut line = incomplete_lines::blue_replacement();
        line.add_color_when_applicable(&colors());
        assert_eq!(line.color, None);
    }
}