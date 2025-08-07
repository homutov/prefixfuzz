use crate::prefix::trie::{Agent, State};

pub struct Matcher {
    results: Vec<(String, u32, usize)>,
    pub term: Vec<char>,
    pub max_dist: usize,
    pub limit: Option<usize>,
}

impl Matcher {
    pub fn new(term: String, max_dist: usize, limit: Option<usize>) -> Matcher {
        Matcher {
            results: Vec::new(),
            term: term.chars().collect(),
            max_dist,
            limit,
        }
    }

    pub fn get_results(&self) -> &Vec<(String, u32, usize)> {
        &self.results
    }
}

impl Agent for Matcher {
    fn initial_state(&mut self) -> State {
        let mut state = State::new();
        state.put("CUR_PREFIX".to_string(), "".chars().collect::<Vec<char>>());
        state
    }

    fn visit_node(&mut self, node_id: u32, payload: Option<u32>, state: &mut State) {
        if state.get::<Vec<char>>("CUR_PREFIX").unwrap().len() == 0 {
            // root node
            state.put(
                "CUR_ROW".to_string(),
                (0..=self.term.len()).collect::<Vec<usize>>(),
            );
            return;
        }

        let cur_prefix = state.get::<Vec<char>>("CUR_PREFIX").unwrap().clone();
        let prev_row = state.get::<Vec<usize>>("PREV_ROW").unwrap().clone();

        let mut cur_row = vec![prev_row[0] + 1];
        for column in 1..=self.term.len() {
            let insert_cost = cur_row[column - 1] + 1;
            let delete_cost = prev_row[column] + 1;

            let replace_cost: usize;
            if self.term[column - 1] != *cur_prefix.last().unwrap() {
                replace_cost = prev_row[column - 1] + 1;
            } else {
                replace_cost = prev_row[column - 1];
            }

            let min_cost = *[insert_cost, delete_cost, replace_cost]
                .iter()
                .min()
                .unwrap();
            cur_row.push(min_cost)
        }

        state.put::<Vec<usize>>("CUR_ROW".to_string(), cur_row.clone());

        // if the last entry in the row indicates the optimal cost is less than the
        // maximum cost, and there is a word in this trie node, then add it.
        if payload.is_some() && *cur_row.last().unwrap() <= self.max_dist {
            if self.limit.is_none() || (self.results.len() < self.limit.unwrap()) {
                self.results.push((
                    cur_prefix.into_iter().collect(),
                    node_id,
                    *cur_row.last().unwrap(),
                ))
            }
        }
    }

    fn get_children_to_visit(
        &mut self,
        _node_id: u32,
        state: &State,
        children: Vec<(char, u32)>,
    ) -> Vec<(u32, State)> {
        let cur_row = state.get::<Vec<usize>>("CUR_ROW").unwrap().clone();
        let cur_prefix = state.get::<Vec<char>>("CUR_PREFIX").unwrap().clone();
        let cur_prefix_len = cur_prefix.len();

        let is_limit_reached = self.limit.is_some() && (self.results.len() >= self.limit.unwrap());
        let is_max_dist_reached = *cur_row.iter().min().unwrap() > self.max_dist;

        let mut exact_child_with_state: Option<(u32, State)> = None;
        let mut other_children_with_state: Vec<(u32, State)> = Vec::new();
        if !is_max_dist_reached && !is_limit_reached {
            for (child_char, child_id) in children {
                let mut child_state = State::new();
                child_state.put("PREV_ROW".to_string(), cur_row.clone());
                let mut child_prefix = cur_prefix.clone();
                child_prefix.push(child_char);
                child_state.put("CUR_PREFIX".to_string(), child_prefix.clone());

                if (self.term.len() > cur_prefix_len) && (child_char == self.term[cur_prefix_len]) {
                    // Heuristic to visit the child with minimal Levenstein distance first
                    exact_child_with_state = Some((child_id, child_state))
                } else {
                    other_children_with_state.push((child_id, child_state))
                }
            }
        }

        let mut children_with_state: Vec<(u32, State)>;
        if let Some((child_id, child_state)) = exact_child_with_state {
            children_with_state = vec![(child_id, child_state)];
            children_with_state.extend(other_children_with_state);
        } else {
            children_with_state = other_children_with_state
        }
        children_with_state
    }
}
