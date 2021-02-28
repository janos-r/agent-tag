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
        moves,
        print_announce: announce_tag,
        disable_grid,
    } = Input::from_args();

    let world = World::new(n_of_agents, size, announce_tag);
    let mut rng = thread_rng();

    for _tick in 0..moves {
        if !disable_grid {
            // clear terminal
            print!("\x1B[2J");
            println!(" __{}__", "__".repeat(size));
            println!("|  {}  |", "__".repeat(size));
            world.borrow().print_grid();
            println!("| |{}| |", "__".repeat(size));
            println!(" __{}__", "__".repeat(size));
            world.borrow().print_tag_count()
        }
        World::tick(&world, &mut rng, disable_grid, sleep_in_millis)
    }
    if disable_grid {
        world.borrow().print_tag_count()
    }
}
