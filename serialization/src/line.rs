use serde_derive::{Deserialize, Serialize};

use super::train::Train;
use simulation::line::Kind;
use simulation::{Color, Node};

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    name: String,
    kind: Kind,
    nodes: Vec<Node>,
    trains: Vec<Train>,
}

impl Line {
    pub fn new(name: String, kind: Kind, nodes: Vec<Node>, trains: Vec<Train>) -> Line {
        Line {
            name,
            kind,
            nodes,
            trains,
        }
    }

    fn unfreeze(self) -> simulation::Line {
        let kind = self.kind;
        let nodes = self.nodes;
        let trains = self
            .trains
            .into_iter()
            .map(|train| train.unfreeze(kind, &nodes))
            .collect();

        simulation::Line::new(self.name, kind, nodes, trains)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineGroup {
    color: Color,
    lines: Vec<Line>,
}

impl LineGroup {
    pub fn new(color: Color, lines: Vec<Line>) -> LineGroup {
        LineGroup { color, lines }
    }

    pub fn unfreeze(self) -> simulation::LineGroup {
        let lines = self.lines.into_iter().map(|line| line.unfreeze()).collect();
        simulation::LineGroup::new(self.color, lines)
    }
}
