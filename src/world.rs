use crate::agent::{Agent, Status};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::{iter::ParallelIterator, prelude::*};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

type Grid = Vec<Vec<Option<Status>>>;

pub struct World {
    pub agents: Vec<Agent>,
    pub grid: Grid,
    pub size: usize,
    pub tag_count: u32,
}
impl World {
    pub fn new(n_of_agents: usize, size: usize, announce_tag: bool) -> Arc<Mutex<World>> {
        let world = World {
            agents: Vec::new(),
            grid: Vec::with_capacity(size),
            size,
            tag_count: 0,
        };

        // create link to world
        let world_link: Arc<Mutex<World>> = Arc::new(Mutex::new(world));

        // generate agents and add to world
        for _ in 0..n_of_agents {
            let agent = Agent::new(
                SmallRng::from_entropy(),
                announce_tag,
                size,
                world_link.clone(),
            );
            world_link.lock().unwrap().agents.push(agent);
        }

        // make one Tagged
        let tag_index = SmallRng::from_entropy().gen_range(0..n_of_agents);
        world_link.lock().unwrap().agents[tag_index].status = Status::Tagged;

        // create grid
        world_link.lock().unwrap().update_grid();

        world_link
    }
    pub fn tick(world: &Arc<Mutex<World>>, disable_grid: bool, sleep_in_millis: u64) {
        world.lock().unwrap().move_agents();

        // regret: this still feels like a hack to me
        // tag agents
        let agents = world.lock().unwrap().agents.clone();
        agents
            .iter()
            .enumerate()
            .for_each(|(index, agent)| agent.tag(index));

        if !disable_grid {
            world.lock().unwrap().update_grid();
        }

        if sleep_in_millis > 0 {
            thread::sleep(Duration::from_millis(sleep_in_millis));
        }
    }

    pub fn update_grid(&mut self) {
        // init empty grid
        let mut new_grid: Grid = (0..self.size)
            .map(|_| (0..self.size).map(|_| None).collect())
            .collect();
        // populate grid with agents
        self.agents.iter().for_each(|agent| {
            new_grid[agent.position.1][agent.position.0] = Some(agent.status.clone());
        });
        self.grid = new_grid;
    }
    // # Raion
    fn move_agents(&mut self) {
        self.agents
            .iter_mut()
            // .par_iter_mut()
            .for_each(|agent| agent.move_position());
    }
    // # Raion
    pub fn tag_agent(&mut self, origin: usize, target: usize) {
        self.tag_count += 1;
        self.agents
            .iter_mut()
            // .par_iter_mut()
            .for_each(|agent| agent.status = Status::Normal);
        self.agents[target].status = Status::Tagged;
        self.agents[origin].status = Status::UnTaggable;
    }
    pub fn print_grid(&self) {
        self.grid.iter().for_each(|row| {
            let line: String = row
                .iter()
                .map(|field| match field {
                    Some(Status::Tagged) => "😈️",
                    Some(Status::UnTaggable) => "😀️",
                    Some(Status::Normal) => "😑️",
                    None => "  ",
                })
                .collect();
            println!("| |{}| |", line)
        });
    }
    pub fn print_tag_count(&self) {
        println!("Total count of exchanged tags: {}", self.tag_count)
    }
}
