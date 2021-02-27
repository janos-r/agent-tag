use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::rc::Rc;
use std::time::Duration;
use std::{cell::RefCell, rc::Weak};
use std::{thread, usize};

/*

struct World
    > message
    > announce_tag (all untaggable > normal)
    > tick - agents_move, agents_tag; update grid

struct Agent
    > find neighbor -> Option index - read from world (need response so can't be message)
    > tag - if tagged
            - find neighbor
                - if found neighbor
                    // mutate world
                    - tag agent
                    // mutate self
                    - untag self (tagged > untaggable)

enum Message
    - tag agent (index)
        - announce tag

input
    - n of agents
    - n of ms per tick

*/

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

        world_link
    }
    fn update_grid(&mut self) {
        // init empty grid
        let mut new_grid: Grid = (0..SIZE)
            .map(|_| (0..SIZE).map(|_| None).collect())
            .collect();
        // populate grid with agents
        self.agents.iter().for_each(|agent| {
            // later agents overlap the previous on the same position
            new_grid[agent.position.1][agent.position.0] = Some(agent.status.clone())
        });
        self.grid = new_grid;
    }
    fn print_grid(&self) {
        self.grid.iter().for_each(|row| {
            let line: String = row
                .iter()
                .map(|field| match field {
                    Some(Status::Tagged) => '#',
                    Some(Status::UnTaggable) => '*',
                    Some(Status::Normal) => 'o',
                    None => '.',
                })
                .collect();
            println!("| |{}| |", line)
        });
    }
    fn tick(&mut self, rng: &mut ThreadRng, sleep_in_millis: u64) {
        // move agents
        self.agents
            .iter_mut()
            .for_each(|agent| agent.move_position(rng));

        // todo: tag agents

        // update grid
        self.update_grid();
        thread::sleep(Duration::from_millis(sleep_in_millis));
    }
}

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

    fn move_position(&mut self, rng: &mut ThreadRng) {
        let direction = rng.gen_range(0..4);
        fn bellow_zero(n: usize) -> usize {
            if n == 0 {
                // last index
                SIZE - 1
            } else {
                n - 1
            }
        }
        match direction {
            // on edges - pop out on the other side
            0 => self.position.0 = (self.position.0 + 1) % (SIZE - 1),
            1 => self.position.0 = bellow_zero(self.position.0),
            2 => self.position.1 = (self.position.1 + 1) % (SIZE - 1),
            _ => self.position.1 = bellow_zero(self.position.1),
        }
    }
}

#[derive(Clone)]
enum Status {
    Normal,
    Tagged,
    UnTaggable,
}

fn main() {
    let world = World::new(6);
    let mut rng = thread_rng();
    let sleep_in_millis = 1000;

    for _tick in 0..10 {
        // clear terminal
        print!("\x1B[2J");
        println!(" ____________________________");
        println!("|  ________________________  |");

        world.borrow_mut().print_grid();

        println!("| |________________________| |");
        println!(" ____________________________");

        world.borrow_mut().tick(&mut rng, sleep_in_millis);
    }
}
