use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::rc::Rc;
use std::time::Duration;
use std::{cell::RefCell, rc::Weak};
use std::{thread, usize};

const SIZE: usize = 24;
type Grid = Vec<Vec<Option<Status>>>;
type Position = (usize, usize);

struct World {
    agents: Vec<Agent>,
    grid: Grid,
}
impl World {
    fn new(n_of_agents: usize) -> Rc<RefCell<World>> {
        let world = World {
            agents: Vec::new(),
            grid: Vec::with_capacity(SIZE),
        };

        // create link to world
        let world_link: Rc<RefCell<World>> = Rc::new(RefCell::new(world));

        // generate agents and add to world
        let mut rng = thread_rng();
        for _ in 0..n_of_agents {
            let agent = Agent::new(&mut rng, &world_link);
            world_link.borrow_mut().agents.push(agent);
        }

        // make one Tagged
        let tag_index = rng.gen_range(0..n_of_agents);
        world_link.borrow_mut().agents[tag_index].status = Status::Tagged;

        // create grid
        world_link.borrow_mut().update_grid();

        world_link
    }
    fn tick(world: &Rc<RefCell<World>>, rng: &mut ThreadRng, sleep_in_millis: u64) {
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

    fn update_grid(&mut self) {
        // init empty grid
        let mut new_grid: Grid = (0..SIZE)
            .map(|_| (0..SIZE).map(|_| None).collect())
            .collect();
        // populate grid with agents
        self.agents.iter().for_each(|agent| {
            new_grid[agent.position.1][agent.position.0] = Some(agent.status.clone())
        });
        self.grid = new_grid;
    }

    fn print_grid(&self) {
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

#[derive(Clone, PartialEq, Debug)]
enum Status {
    Normal,
    Tagged,
    UnTaggable,
}

#[derive(Clone)]
struct Agent {
    position: Position,
    status: Status,
    world: Weak<RefCell<World>>,
}
impl Agent {
    fn new(rng: &mut ThreadRng, world: &Rc<RefCell<World>>) -> Self {
        Agent {
            position: (rng.gen_range(0..SIZE), rng.gen_range(0..SIZE)),
            status: Status::Normal,
            world: Rc::downgrade(world),
        }
    }
    fn position_sub(n: usize) -> usize {
        if n == 0 {
            // last index
            SIZE - 1
        } else {
            n - 1
        }
    }
    fn position_add(n: usize) -> usize {
        (n + 1) % (SIZE - 1)
    }

    fn move_position(&mut self, rng: &mut ThreadRng) {
        let direction = rng.gen_range(0..4);
        match direction {
            // on edges - pop out on the other side
            0 => self.position.0 = Agent::position_add(self.position.0),
            1 => self.position.0 = Agent::position_sub(self.position.0),
            2 => self.position.1 = Agent::position_add(self.position.1),
            _ => self.position.1 = Agent::position_sub(self.position.1),
        }
    }
    fn find_neighbor(&self) -> Option<usize> {
        self.world
            .upgrade()
            .expect("couldn't upgrade")
            // regret: don't know why just '.borrow()' doesn't work - can't infer type parameter
            .borrow_mut()
            .agents
            .iter()
            .position(|agent| {
                let neighbors = [
                    (Agent::position_add(self.position.0), self.position.1),
                    (Agent::position_sub(self.position.0), self.position.1),
                    (self.position.0, Agent::position_add(self.position.1)),
                    (self.position.0, Agent::position_sub(self.position.1)),
                ];
                neighbors.iter().any(|&neighbor| neighbor == agent.position)
            })
    }
    fn tag(&self, my_index: usize) {
        // this is a cloned self, so changes on it won't influence the real world!
        // only it's links or current state are useful
        if self.status == Status::Tagged {
            if let Some(index) = self.find_neighbor() {
                println!("!!!! FOUND NEIGHBOR !!!!");
                // announce tag
                self.world
                    .upgrade()
                    .expect("couldn't upgrade")
                    .borrow_mut()
                    .agents
                    .iter_mut()
                    .for_each(|agent| agent.status = Status::Normal);

                // mutate neighbor
                self.world
                    .upgrade()
                    .expect("couldn't upgrade")
                    .borrow_mut()
                    .agents[index]
                    .status = Status::Tagged;

                // mutate self
                self.world
                    .upgrade()
                    .expect("couldn't upgrade")
                    .borrow_mut()
                    .agents[my_index]
                    .status = Status::UnTaggable;
            }
        };
    }
}

fn main() {
    // inputs
    let sleep_in_millis = 1000;
    let n_of_agents = 40;

    let world = World::new(n_of_agents);
    let mut rng = thread_rng();

    for _tick in 0..10 {
        // clear terminal
        print!("\x1B[2J");
        println!(" __{}__", "__".repeat(SIZE));
        println!("|  {}  |", "__".repeat(SIZE));
        world.borrow().print_grid();
        println!("| |{}| |", "__".repeat(SIZE));
        println!(" __{}__", "__".repeat(SIZE));

        World::tick(&world, &mut rng, sleep_in_millis)
    }
}
