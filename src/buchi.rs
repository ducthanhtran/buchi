use std::collections::HashSet;

struct State {
    id: usize,
    is_accepting: bool
}

struct Buchi {
    states: Vec<State>,
    init_state: usize,
    transitions: Vec<Vec<(usize, char)>>
}

struct BuchiBuilder {
    num_states: usize,
    initial_state: Option<usize>,
    accepting_states: HashSet<usize>,
    transitions: Vec<Vec<(usize, char)>>
}

impl BuchiBuilder {
    fn new(num_states: usize) -> Self {
        BuchiBuilder {
            num_states,
            initial_state: None,
            accepting_states: HashSet::new(),
            transitions: vec![Vec::new(); num_states]
        }
    }
}