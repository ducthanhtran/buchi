use std::collections::HashSet;

#[derive(Debug)] 
pub struct State {
    id: usize,
    is_accepting: bool
}

#[derive(Debug)] 
pub struct Buchi {
    states: Vec<State>,
    init_state: usize,
    transitions: Vec<Vec<(usize, char)>>
}

#[derive(Debug)] 
pub struct BuchiBuilder {
    num_states: usize,
    initial_state: Option<usize>,
    accepting_states: HashSet<usize>,
    transitions: Vec<Vec<(usize, char)>>
}

impl BuchiBuilder {
    pub fn new(num_states: usize) -> Self {
        BuchiBuilder {
            num_states,
            initial_state: None,
            accepting_states: HashSet::new(),
            transitions: vec![Vec::new(); num_states]
        }
    }

    pub fn build(self) -> Result<Buchi, String> {
        let init_state = self.initial_state.ok_or_else(|| "Initial state not set".to_string())?;
        
        if self.accepting_states.is_empty() {
            return Err("At least one accepting state required.".to_string());
        }

        let states = (0..self.num_states)
            .map(|id| State {
                id,
                is_accepting: self.accepting_states.contains(&id)
            }).collect();
        
        Ok(Buchi {
            states,
            init_state,
            transitions: self.transitions
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_no_initial_state() {
        let result = BuchiBuilder::new(1).build();
        assert_eq!(result.unwrap_err(), "Initial state not set");
    }
}