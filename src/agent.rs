use crate::world::World;
use rand::{prelude::ThreadRng, Rng};
use std::rc::Rc;
use std::{cell::RefCell, rc::Weak};

type Position = (usize, usize);

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Normal,
    Tagged,
    UnTaggable,
}

#[derive(Clone)]
pub struct Agent {
    pub position: Position,
    pub status: Status,
    world_size: usize,
    world: Weak<RefCell<World>>,
}
impl Agent {
    pub fn new(rng: &mut ThreadRng, world_size: usize, world: &Rc<RefCell<World>>) -> Self {
        Agent {
            position: (rng.gen_range(0..world_size), rng.gen_range(0..world_size)),
            status: Status::Normal,
            world_size,
            world: Rc::downgrade(world),
        }
    }
    fn position_sub(&self, n: usize) -> usize {
        if n == 0 {
            // last index
            self.world_size - 1
        } else {
            n - 1
        }
    }
    fn position_add(&self, n: usize) -> usize {
        (n + 1) % (self.world_size - 1)
    }

    pub fn move_position(&mut self, rng: &mut ThreadRng) {
        let direction = rng.gen_range(0..4);
        match direction {
            // on edges - pop out on the other side
            0 => self.position.0 = self.position_add(self.position.0),
            1 => self.position.0 = self.position_sub(self.position.0),
            2 => self.position.1 = self.position_add(self.position.1),
            _ => self.position.1 = self.position_sub(self.position.1),
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
                    (self.position_add(self.position.0), self.position.1),
                    (self.position_sub(self.position.0), self.position.1),
                    (self.position.0, self.position_add(self.position.1)),
                    (self.position.0, self.position_sub(self.position.1)),
                ];
                neighbors.iter().any(|&neighbor_position| {
                    neighbor_position == agent.position && agent.status != Status::UnTaggable
                })
            })
    }
    pub fn tag(&self, my_index: usize) {
        // this is a cloned self, so changes on it won't influence the real world!
        // only it's links or current state are useful
        if self.status == Status::Tagged {
            if let Some(target) = self.find_neighbor() {
                println!("!!!! FOUND NEIGHBOR !!!!");
                self.world
                    .upgrade()
                    .expect("couldn't upgrade")
                    .borrow_mut()
                    .tag_agent(my_index, target);
            }
        };
    }
}