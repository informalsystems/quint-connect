use crate::system::*;
use quint_connect::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

type NodeId = String;

#[derive(Eq, PartialEq, Deserialize, Debug)]
struct SpecState(BTreeMap<NodeId, ProcState>);

#[derive(Eq, PartialEq, Deserialize, Debug)]
struct ProcState {
    stage: Stage,
}

impl State<TwoPhaseCommitDriver> for SpecState {
    fn from_driver(driver: &TwoPhaseCommitDriver) -> Result<Self> {
        let procs = driver
            .processes
            .iter()
            .map(|(id, node)| {
                let stage = node.stage();
                (id.clone(), ProcState { stage })
            })
            .collect();

        Ok(Self(procs))
    }
}

#[derive(Default)]
struct TwoPhaseCommitDriver {
    processes: BTreeMap<NodeId, Node>,
    messages_sent: BTreeMap<NodeId, BTreeSet<Message>>,
}

impl Driver for TwoPhaseCommitDriver {
    type State = SpecState;

    fn annotations() -> SpecAnnotations {
        SpecAnnotations {
            state_location: &["two_phase_commit::choreo::s", "system"],
            nondet_location: &["two_phase_commit::choreo::s", "extensions", "actionTaken"],
        }
    }

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            Init => self.init(),
            SpontaneouslyPrepares(node) => self.spontaneously_prepares(node),
            SpontaneouslyAborts(node) => self.spontaneously_aborts(node),
            AbortsAsInstructed(node) => self.aborts_as_instructed(node),
            CommitsAsInstructed(node) => self.commits_as_instructed(node),
            DecidesOnCommit(node) => self.decides_on_commit(node),
            DecidesOnAbort(node) => self.decides_on_abort(node)
        })
    }
}

impl TwoPhaseCommitDriver {
    fn init(&mut self) {
        self.processes.clear();
        self.messages_sent.clear();

        for id in ["c", "p1", "p2", "p3"] {
            let mut msgs = BTreeSet::new();
            let node = if id == "c" {
                let coord = Coordinator::new(3);
                msgs.insert(coord.start());
                Node::Coordinator(coord)
            } else {
                Node::Participant(Participant::default())
            };
            self.processes.insert(id.to_string(), node);
            self.messages_sent.insert(id.to_string(), msgs);
        }
    }

    fn spontaneously_prepares(&mut self, node: NodeId) {
        let prepare_msg = self.prepare_msg();
        let (node, msgs) = self.locate_node(node);
        Self::record_reply(msgs, node.receive(prepare_msg));
    }

    fn spontaneously_aborts(&mut self, node: NodeId) {
        let (node, msgs) = self.locate_node(node);
        Self::record_reply(msgs, node.timeout());
    }

    fn aborts_as_instructed(&mut self, node: NodeId) {
        let abort_msg = self.abort_msg();
        let (node, msgs) = self.locate_node(node);
        Self::record_reply(msgs, node.receive(abort_msg));
    }

    fn commits_as_instructed(&mut self, node: NodeId) {
        let commit_msg = self.commit_msg();
        let (node, msgs) = self.locate_node(node);
        Self::record_reply(msgs, node.receive(commit_msg));
    }

    fn decides_on_commit(&mut self, node: NodeId) {
        let prepared_msgs = self.prepared_msgs();
        let (node, msgs) = self.locate_node(node);
        for msg in prepared_msgs {
            Self::record_reply(msgs, node.receive(msg));
        }
    }

    fn decides_on_abort(&mut self, node: NodeId) {
        let (node, msgs) = self.locate_node(node);
        Self::record_reply(msgs, node.timeout());
    }

    fn locate_node(&mut self, node_id: NodeId) -> (&mut Node, &mut BTreeSet<Message>) {
        let node = self.processes.get_mut(&node_id).unwrap();
        let messages = self.messages_sent.get_mut(&node_id).unwrap();
        (node, messages)
    }

    fn record_reply(msgs: &mut BTreeSet<Message>, reply: Option<Message>) {
        if let Some(reply) = reply {
            msgs.insert(reply);
        }
    }

    fn prepare_msg(&self) -> Message {
        *self
            .messages_sent
            .get(&"c".to_string())
            .unwrap()
            .iter()
            .find(|msg| matches!(msg, Message::Prepare))
            .unwrap()
    }

    fn prepared_msgs(&self) -> Vec<Message> {
        self.messages_sent
            .iter()
            .flat_map(|(_, msgs)| {
                msgs.iter()
                    .filter(|msg| matches!(msg, Message::Prepared))
                    .cloned()
            })
            .collect()
    }

    fn commit_msg(&self) -> Message {
        *self
            .messages_sent
            .get(&"c".to_string())
            .unwrap()
            .iter()
            .find(|msg| matches!(msg, Message::Commit))
            .unwrap()
    }

    fn abort_msg(&self) -> Message {
        *self
            .messages_sent
            .get(&"c".to_string())
            .unwrap()
            .iter()
            .find(|msg| matches!(msg, Message::Abort))
            .unwrap()
    }
}

#[quint_run(
    spec = "examples/two_phase_commit/spec/two_phase_commit.qnt",
    max_samples = 1
)]
fn test_simulation() -> impl Driver {
    TwoPhaseCommitDriver::default()
}

#[quint_test(
    spec = "examples/two_phase_commit/spec/two_phase_commit.qnt",
    test = "commitTest"
)]
fn test_commit() -> impl Driver {
    TwoPhaseCommitDriver::default()
}
