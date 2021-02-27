use std::rc::Rc;
use std::time::Duration;
use std::{cell::RefCell, rc::Weak};
use std::{thread, usize};

/*

struct World
    - agents
    - grid
    > update_grid
    > message
    > announce_tag (all untaggable > normal)
    > tick - agents_move, agents_tag; update grid
    > print

struct Agent
    - position
    - state - tagged, untaggable, normal
    - &world
    > new
    > move
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
    fn new() -> Self {
        Self {
            agents: Vec::new(),
            grid: Vec::with_capacity(SIZE),
        }
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
}

struct Agent {
    position: Position,
    status: Status,
}

#[derive(Clone)]
enum Status {
    Normal,
    Tagged,
    UnTaggable,
}

fn main() {
    // create world
    let world = Rc::new(RefCell::new(World::new()));

    // generate agents and add to world
    let a1 = Agent {
        position: (0, 0),
        status: Status::Normal,
    };
    let a2 = Agent {
        position: (1, 1),
        status: Status::Normal,
    };
    world.borrow_mut().agents.push(a1);
    world.borrow_mut().agents.push(a2);

    fn get_line_string(line: &Vec<Option<Status>>) -> String {
        line.iter()
            .map(|field| match field {
                Some(_) => '#',
                None => '.',
            })
            .collect()
    }

    for _tick in 0..10 {
        // clear terminal
        print!("\x1B[2J");
        println!(" ____________________________");
        println!("|  ________________________  |");

        world.borrow_mut().update_grid();

        // print world
        world
            .borrow()
            .grid
            .iter()
            .for_each(|row| println!("| |{}| |", get_line_string(row)));

        println!("| |________________________| |");
        println!(" ____________________________");

        // move
        world.borrow_mut().agents[0].position.0 += 1;
        world.borrow_mut().agents[1].position.1 += 1;
        thread::sleep(Duration::from_millis(1000));
    }
}
