
extern crate rand;

mod agent;

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


fn main() {
    let mut rng = rand::thread_rng();

    let mut agent = Agent::new(&mut rng);

    let mut array = gen(&mut rng);
    println!("{:?}", array);

    match agent.act() {
        Action::Act => {
            let (i1, i2) = pick_indices(&mut rng, &agent.index_affinity);
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

    println!("{:?}", array);
    println!("{:?}", agent);
}
