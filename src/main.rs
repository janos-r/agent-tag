use structopt::StructOpt;
mod agent;
mod input;
mod world;
use input::Input;
use rand::thread_rng;
use world::World;

fn main() {
    let Input {
        agents: n_of_agents,
        time: sleep_in_millis,
        size,
    } = Input::from_args();

    let world = World::new(n_of_agents, size);
    let mut rng = thread_rng();

    for _tick in 0..10 {
        // clear terminal
        print!("\x1B[2J");
        println!(" __{}__", "__".repeat(size));
        println!("|  {}  |", "__".repeat(size));
        world.borrow().print_grid();
        println!("| |{}| |", "__".repeat(size));
        println!(" __{}__", "__".repeat(size));

        World::tick(&world, &mut rng, sleep_in_millis)
    }
}
