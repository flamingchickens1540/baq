use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct QueueState {
    team_list: HashMap<String, Vec<Team>>,
    queue: Vec<String>,
}

impl QueueState {
    pub fn get_queue(&self) -> &Vec<String> {
        &self.queue
    }

    pub fn queue_robot(&mut self, team: impl ToString) -> &Vec<String> {
        let played_with = self.team_list.entry(team.to_string()).or_insert(Vec::new());

        let mut teams_count = HashMap::new();
        played_with.iter().for_each(|team| {
            teams_count
                .entry(team)
                .and_modify(|n| {
                    *n += 1;
                })
                .or_insert(1);
        });

        let mut teams: Vec<(&String, i32)> = teams_count.into_iter().collect();
        teams.sort_by(|(_name, count), (_name_1, count_1)| count.cmp(count_1));
        let mut teams: Vec<String> = teams.into_iter().map(|x| x.0.clone()).collect();

        let n = 2;
        let top_n: Vec<String> = teams.split_off(n);

        let (_rest_queue, bottom_6_queue) = self.queue.split_at(self.queue.len() - 6 - 1);

        // Teams that `team` has played with frequently and that it would have been slated to play
        // with again
        let _problem_teams =
            top_n
                .iter()
                .filter_map(|team_played| match bottom_6_queue.contains(team_played) {
                    true => Some(team_played),
                    false => None,
                });

        // TODO Figure out how to rearrange the queue

        self.queue.push(team.to_string());

        &self.queue
    }
}

pub type Team = String;
