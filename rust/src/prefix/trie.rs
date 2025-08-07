use std::any::Any;
use std::collections::HashMap;
use bincode::{Decode, Encode};

pub struct State {
    map: HashMap<String, Box<dyn Any>>,
}

impl State {
    pub fn new() -> Self {
        State {
            map: HashMap::new(),
        }
    }

    pub fn put<T: 'static>(&mut self, attr: String, value: T) {
        self.map.insert(attr, Box::new(value));
    }

    pub fn get<T: 'static>(&self, attr: &str) -> Option<&T> {
        self.map
            .get(attr)
            .and_then(|value| value.downcast_ref::<T>())
    }
}

pub trait Agent {
    fn initial_state(&mut self) -> State;
    fn visit_node(&mut self, node_id: u32, payload: Option<u32>, state: &mut State);
    fn get_children_to_visit(
        &mut self,
        node_id: u32,
        state: &State,
        children: Vec<(char, u32)>,
    ) -> Vec<(u32, State)>;
}


#[derive(Encode, Decode)]
pub struct Trie {
    node_shifts: Vec<u32>,
    node_chars: Vec<Option<char>>,
    node_payloads: Vec<Option<u32>>,
    child_indices: Vec<u32>,
}

impl Trie {
    pub fn from_internal_data(
        node_shifts: Vec<u32>,
        node_chars: Vec<Option<char>>,
        node_payloads: Vec<Option<u32>>,
        child_indices: Vec<u32>,
    ) -> Trie {
        Trie {
            node_shifts,
            node_chars,
            node_payloads,
            child_indices,
        }
    }

    pub fn get_children(&self, node_id: u32) -> Vec<(char, u32)> {
        let from_shift = self.node_shifts[node_id as usize];

        let to_shift: u32;
        if node_id + 1 < self.node_shifts.len() as u32 {
            to_shift = self.node_shifts[(node_id + 1) as usize];
        } else {
            to_shift = self.child_indices.len() as u32;
        }

        let mut children = Vec::new();
        for child_shift in from_shift..to_shift {
            let child_id = self.child_indices[child_shift as usize];
            children.push((self.node_chars[child_id as usize].unwrap(), child_id))
        }

        children
    }

    pub fn dfs<A: Agent>(&self, agent: &mut A) {
        let initial_state = agent.initial_state();

        let mut stack: Vec<(u32, State)> = Vec::new();
        stack.push((0, initial_state));

        while !stack.is_empty() {
            let (node_id, mut state) = stack.pop().unwrap();
            agent.visit_node(node_id, self.node_payloads[node_id as usize], &mut state);

            let children = self.get_children(node_id);

            let mut children_to_visit = agent.get_children_to_visit(node_id, &state, children);
            children_to_visit.reverse();
            for (child_id, child_state) in children_to_visit {
                stack.push((child_id, child_state))
            }
        }
    }

    pub fn get_payload(&self, node_id: u32) -> Option<u32> {
        self.node_payloads[node_id as usize]
    }
}
