use std::thread;
use std::time::Duration;

enum Agent {
    Normal,
    _Tagged,
}

type World = Vec<Vec<Option<Agent>>>;

fn main() {
    const SIZE: usize = 24;
    let mut a1 = (0, 0);
    let mut world: World = Vec::with_capacity(SIZE);

    fn create_world(world: &mut World, size: usize, agent: (usize, usize)) {
        let mut new_world: World = (0..size)
            .map(|_| (0..size).map(|_| None).collect())
            .collect();
        // update world with agents
        new_world[agent.0][agent.1] = Some(Agent::Normal);
        *world = new_world;
    }

    fn get_line_string(line: &Vec<Option<Agent>>) -> String {
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

        create_world(&mut world, SIZE, a1);

        // print world
        world
            .iter()
            .for_each(|row| println!("| |{}| |", get_line_string(row)));

        println!("| |________________________| |");
        println!(" ____________________________");

        a1.1 += 1;
        thread::sleep(Duration::from_millis(1000));
    }
}
