use serde::Deserialize;

#[derive(Eq, PartialEq, Deserialize, Default, Copy, Clone, Debug)]
#[serde(tag = "tag")]
pub enum Stage {
    #[default]
    Working,
    Prepared,
    Committed,
    Aborted,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Message {
    Prepare,
    Prepared,
    Commit,
    Abort,
}

pub enum Node {
    Coordinator(Coordinator),
    Participant(Participant),
}

impl Node {
    pub fn stage(&self) -> Stage {
        match self {
            Node::Coordinator(coord) => coord.stage,
            Node::Participant(part) => part.stage,
        }
    }

    pub fn timeout(&mut self) -> Option<Message> {
        match self {
            Node::Coordinator(coord) => coord.timeout(),
            Node::Participant(part) => part.timeout(),
        }
    }

    pub fn receive(&mut self, msg: Message) -> Option<Message> {
        match self {
            Node::Coordinator(coord) => coord.receive(msg),
            Node::Participant(part) => part.receive(msg),
        }
    }
}

pub struct Coordinator {
    stage: Stage,
    quorum: usize,
    prepared: usize,
}

impl Coordinator {
    pub fn new(quorum: usize) -> Self {
        Self {
            stage: Stage::Working,
            prepared: 0,
            quorum,
        }
    }

    pub fn start(&self) -> Message {
        Message::Prepare
    }

    pub fn timeout(&mut self) -> Option<Message> {
        match self.stage {
            Stage::Working => {
                self.stage = Stage::Aborted;
                Some(Message::Abort)
            }
            _ => None,
        }
    }

    pub fn receive(&mut self, msg: Message) -> Option<Message> {
        match (self.stage, msg) {
            (Stage::Working, Message::Prepared) => {
                self.prepared += 1;
                if self.prepared == self.quorum {
                    self.stage = Stage::Committed;
                    return Some(Message::Commit);
                }
                None
            }
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Participant {
    stage: Stage,
}

impl Participant {
    pub fn timeout(&mut self) -> Option<Message> {
        match self.stage {
            Stage::Working => {
                self.stage = Stage::Aborted;
                None
            }
            _ => None,
        }
    }

    pub fn receive(&mut self, msg: Message) -> Option<Message> {
        match (self.stage, msg) {
            (Stage::Working, Message::Prepare) => {
                self.stage = Stage::Prepared;
                Some(Message::Prepared)
            }
            (_, Message::Abort) => {
                self.stage = Stage::Aborted;
                None
            }
            (_, Message::Commit) => {
                self.stage = Stage::Committed;
                None
            }
            _ => None,
        }
    }
}
