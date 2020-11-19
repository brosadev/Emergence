#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ID {
	Nothing,
	Ant,
	Plant,
	Fungus,
}

#[derive(Debug, Clone, Copy)]
pub enum SignalType {
	Passive(ID),
	Push(ID),
	Pull(ID),
	Work,
}
