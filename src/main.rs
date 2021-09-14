
extern crate rand;

mod agent;
mod util;

use rand::Rng;

use agent::{Agent, SortAbility, Action};

fn gen<T : Rng>(rng : &mut T) -> [u8; 8] {
    let mut ret : [u8; 8] = [0; 8];
    for (index, value) in (0..8).map( |i| (i, rng.gen_range(0..8)) ) {
        ret[index] = value
    }
    ret
}

fn pick_indices<T:Rng>( rng : &mut T, options : &Vec<u8> ) -> (u8, u8) {
    let i1 = options[rng.gen_range(0..options.len()) as usize];
    let mut i2 = 0;

    loop {
        i2 = options[rng.gen_range(0..options.len()) as usize];

        if i1 != i2 {
            break;
        }
    }

    (i1, i2)
}

fn proc<T : Rng>( rng : &mut T, agent : &mut Agent, array : &mut [u8] ) {
    match agent.act() {
        Action::Act => {
            let (i1, i2) = pick_indices(rng, &agent.index_affinity);
            let v1 = array[i1 as usize];
            let v2 = array[i2 as usize];
            if agent.sort_ability.flip(i1, i2, v1, v2) {
                let temp = array[i1 as usize];
                array[i1 as usize] = array[i2 as usize];
                array[i2 as usize] = temp;
            }
            agent.index_affinity.push(i1);
            agent.index_affinity.push(i2);
        },
        Action::Observe => panic!("observe"),
        Action::Influence => panic!("influence"),
    }
}

fn prune<T : Rng>(rng : &mut T, agents : &mut Vec<Agent>, count : usize) {
    for _ in 0..count {
        let target_index = rng.gen_range(0..agents.len());
        agents.remove(target_index);
    }
}

fn failure_rate(orig : [u8; 8], attempt : [u8; 8]) -> f32 {
    let mut sorted = orig.clone();
    sorted.sort();

    let mut rev = sorted.clone();
    rev.reverse();

    util::lev(&attempt, &sorted) as f32 / util::lev(&sorted, &rev) as f32
}

fn main() {
    const agent_count : usize = 30;

    let mut rng = rand::thread_rng();

    let mut agents : Vec<Agent> = (0..agent_count).map( |_| Agent::new(&mut rng) ).collect();

    let array = gen(&mut rng);
    let mut attempt = array.clone();
    
    for mut agent in &mut agents {
        proc(&mut rng, &mut agent, &mut attempt);
    }

    prune(&mut rng, &mut agents, (failure_rate(array, attempt) * agent_count as f32) as usize);

    println!("{:?}", agents.len());

}
