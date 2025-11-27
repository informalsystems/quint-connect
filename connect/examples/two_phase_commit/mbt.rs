use crate::system::*;
use quint_connect::*;
use std::collections::BTreeMap;

type NodeId = String;

#[derive(Default)]
struct TwoPhaseCommit {
    process: BTreeMap<NodeId, Node>,
}

impl Driver for TwoPhaseCommit {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step { init })
    }
}

impl TwoPhaseCommit {
    fn init(&mut self) {
        let coord = Node::Coordinator(Coordinator::new(3));
        self.process.insert("c".to_string(), coord);

        for p in ["p1", "p2", "p3"] {
            let part = Node::Participant(Participant::new());
            self.process.insert(p.to_string(), part);
        }
    }
}

#[quint_run(
    spec = "examples/two_phase_commit/spec/two_phase_commit.qnt",
    max_samples = 1
)]
fn test_simulation() -> impl Driver {
    TwoPhaseCommit::default()
}
