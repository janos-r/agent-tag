use crate::agent::{Agent, Status};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::rc::Rc;
use std::{cell::RefCell, thread, time::Duration};

type Grid = Vec<Vec<Option<Status>>>;

pub struct World {
    pub agents: Vec<Agent>,
    pub grid: Grid,
    pub size: usize,
}
impl World {
    pub fn new(n_of_agents: usize, size: usize) -> Rc<RefCell<World>> {
        let world = World {
            agents: Vec::new(),
            grid: Vec::with_capacity(size),
            size,
        };

        // create link to world
        let world_link: Rc<RefCell<World>> = Rc::new(RefCell::new(world));

        // generate agents and add to world
        let mut rng = thread_rng();
        for _ in 0..n_of_agents {
            let agent = Agent::new(&mut rng, size, &world_link);
            world_link.borrow_mut().agents.push(agent);
        }

        // make one Tagged
        let tag_index = rng.gen_range(0..n_of_agents);
        world_link.borrow_mut().agents[tag_index].status = Status::Tagged;

        // create grid
        world_link.borrow_mut().update_grid();

        world_link
    }
    pub fn tick(world: &Rc<RefCell<World>>, rng: &mut ThreadRng, sleep_in_millis: u64) {
        // move agents
        world
            .borrow_mut()
            .agents
            .iter_mut()
            .for_each(|agent| agent.move_position(rng));

        // regret: this still feels like a hack to me
        // tag agents
        let agents = world.borrow().agents.clone();
        agents
            .iter()
            .enumerate()
            .for_each(|(index, agent)| agent.tag(index));

        // update grid
        world.borrow_mut().update_grid();
        thread::sleep(Duration::from_millis(sleep_in_millis));
    }

    pub fn update_grid(&mut self) {
        // init empty grid
        let mut new_grid: Grid = (0..self.size)
            .map(|_| (0..self.size).map(|_| None).collect())
            .collect();
        // populate grid with agents
        self.agents.iter().for_each(|agent| {
            new_grid[agent.position.1][agent.position.0] = Some(agent.status.clone())
        });
        self.grid = new_grid;
    }
    pub fn tag_agent(&mut self, origin: usize, target: usize) {
        self.agents
            .iter_mut()
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
}
