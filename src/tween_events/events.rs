use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(u64)]
pub enum TweenCompletedEvent {
    JumpFinished = 0,
}

impl TweenCompletedEvent {
    pub fn ordinal(&self) -> u64 {
        Ordinalize::ordinal(self)
    }
}
