use std::collections::{HashMap, VecDeque};

#[derive(Clone, Default)]
pub struct QueueState {
    teams: Vec<String>,
    team_to_index: HashMap<&'static str, usize>,
    conflict_matrix: &'static [&'static [u8]],
    queue: VecDeque<String>, // NOTE Not making this a reference cause we insert a request-given buffer
}

impl QueueState {
    pub fn get_queue(&self) -> Vec<String> {
        self.queue.iter().map(|str| str.to_string()).collect()
    }

    pub fn pop_6(&mut self) {
        for _ in 0..6 {
            self.queue.pop_front();
        }
    }

    pub fn insert_robot(&mut self, team: impl ToString) {
        let team = team.to_string();
        let team_num = self.team_to_index.get(team.as_str()).unwrap();
        let mut best_index = 0;
        let mut best_score = u8::MAX;

        self.teams
            .windows(6)
            .enumerate()
            .for_each(|(i, neighbors)| {
                let score = neighbors
                    .iter()
                    .map(|neighbor| {
                        self.conflict_matrix[*team_num]
                            [*self.team_to_index.get(neighbor.as_str()).unwrap()]
                    })
                    .sum::<u8>();

                if score < best_score {
                    best_score = score;
                    best_index = i + 3;
                }
            });
        self.queue.insert(best_index, team);
    }

    pub fn dequeue_robot(&mut self, team: &str) -> Result<(), ()> {
        let index = match self.queue.iter().position(|x| *x == team) {
            Some(i) => i,
            None => return Err(()),
        };
        self.queue.remove(index);

        Ok(())
    }
}
