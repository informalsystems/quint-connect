#[derive(Copy, Clone)]
pub enum Stage {
    Working,
    Prepared,
    Committed,
    Aborted,
}

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
    pub fn timeout(&mut self) -> Vec<Message> {
        match self {
            Node::Coordinator(coord) => coord.timeout(),
            Node::Participant(part) => part.timeout(),
        }
    }

    pub fn receive(&mut self, msg: Message) -> Vec<Message> {
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

    pub fn start(&mut self) -> Message {
        Message::Prepare
    }

    fn timeout(&mut self) -> Vec<Message> {
        match self.stage {
            Stage::Working => {
                self.stage = Stage::Aborted;
                vec![Message::Abort]
            }
            _ => unreachable!(),
        }
    }

    fn receive(&mut self, msg: Message) -> Vec<Message> {
        match (self.stage, msg) {
            (Stage::Working, Message::Prepare) => {
                self.prepared += 1;
                if self.prepared == self.quorum {
                    self.stage = Stage::Committed;
                    return vec![Message::Commit];
                }
                vec![]
            }
            _ => unreachable!(),
        }
    }
}

pub struct Participant {
    stage: Stage,
}

impl Participant {
    pub fn new() -> Self {
        Self {
            stage: Stage::Working,
        }
    }

    fn timeout(&mut self) -> Vec<Message> {
        match self.stage {
            Stage::Working => {
                self.stage = Stage::Aborted;
                vec![]
            }
            _ => unreachable!(),
        }
    }

    fn receive(&mut self, msg: Message) -> Vec<Message> {
        match (self.stage, msg) {
            (Stage::Working, Message::Prepare) => {
                self.stage = Stage::Prepared;
                vec![Message::Prepared]
            }
            (_, Message::Abort) => {
                self.stage = Stage::Aborted;
                vec![]
            }
            (_, Message::Commit) => {
                self.stage = Stage::Committed;
                vec![]
            }
            _ => unreachable!(),
        }
    }
}
