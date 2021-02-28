use crate::world::World;
use rand::{prelude::SmallRng, Rng};
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Normal,
    Tagged,
    UnTaggable,
}

#[derive(Clone)]
pub struct Agent {
    pub position: (usize, usize),
    pub status: Status,
    rng: SmallRng,
    announce_tag: bool,
    world_size: usize,
    world_link: Arc<Mutex<World>>,
}
impl Agent {
    pub fn new(
        mut rng: SmallRng,
        announce_tag: bool,
        world_size: usize,
        world: Arc<Mutex<World>>,
    ) -> Self {
        Agent {
            position: (rng.gen_range(0..world_size), rng.gen_range(0..world_size)),
            status: Status::Normal,
            rng,
            announce_tag,
            world_size,
            world_link: world.clone(),
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

    pub fn move_position(&mut self) {
        let direction = self.rng.gen_range(0..4);
        match direction {
            // on edges - pop out on the other side
            0 => self.position.0 = self.position_add(self.position.0),
            1 => self.position.0 = self.position_sub(self.position.0),
            2 => self.position.1 = self.position_add(self.position.1),
            _ => self.position.1 = self.position_sub(self.position.1),
        }
    }
    fn find_neighbor(&self) -> Option<usize> {
        let neighbors = [
            (self.position_add(self.position.0), self.position.1),
            (self.position_sub(self.position.0), self.position.1),
            (self.position.0, self.position_add(self.position.1)),
            (self.position.0, self.position_sub(self.position.1)),
        ];
        self.world_link
            .lock()
            .unwrap()
            .agents
            .iter()
            .position(|agent| {
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
                if self.announce_tag {
                    println!("!!!! FOUND NEIGHBOR !!!!");
                }
                self.world_link.lock().unwrap().tag_agent(my_index, target);
            }
        };
    }
}
