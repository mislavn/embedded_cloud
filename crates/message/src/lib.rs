use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct Payload {
    pub message: Message,
    pub qos: QoS,
    pub id: Id,
}

#[derive(Arbitrary, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Id {
    Trace(u128),
    Hash([u128; 2]),
}

#[derive(Arbitrary, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: Vec<u8>,
}
