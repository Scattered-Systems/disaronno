use crate::networking::peers::Peer;

pub trait Node {
    fn new() -> Self;
    fn setup() -> Self;
    fn scaffold_chain() -> String;
}

pub struct Interface {
    pub peer: Peer,
}

impl Node for Interface {
    fn new() -> Self {
        todo!()
    }

    fn setup() -> Self {
        todo!()
    }

    fn scaffold_chain() -> String {
        todo!()
    }
}