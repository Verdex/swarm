
use rand::Rng;

#[derive(Debug)]
pub enum Action {
    Observe,
    Influence,
    Act,
}

#[derive(Debug)]
pub struct SortAbility {
    index_1 : u8,
    index_2 : u8,
    value_1 : u8,
    value_2 : u8,
}

fn index_to_bit(index : u8) -> u8 {
    1 << index
}

fn value_to_bit(index : u8) -> u8 {
    1 << index
}

impl SortAbility {
    pub fn flip(&self, i1 : u8, i2 : u8, v1 : u8, v2 : u8) -> bool {
        let index_1 = (index_to_bit(i1) & self.index_1) != 0; 
        let index_2 = (index_to_bit(i2) & self.index_2) != 0;
        let value_1 = (value_to_bit(v1) & self.value_1) != 0;
        let value_2 = (value_to_bit(v2) & self.value_2) != 0;

        index_1 && index_2 && value_1 && value_2
    }

    pub fn move_towards(&mut self, goal : &Self, dist : u8) {
        fn new_value(src : u8, dest : u8, max : u8) -> u8 {
            use std::cmp::min;
            let magnitude = min((dest as i16 - src as i16).abs() as u8, max);
            if dest < src {
                dest + magnitude
            }
            else {
                dest - magnitude
            }
        }

        self.index_1 = new_value(self.index_1, goal.index_1, dist);
        self.index_2 = new_value(self.index_2, goal.index_2, dist);
        self.value_1 = new_value(self.value_1, goal.value_1, dist);
        self.value_2 = new_value(self.value_2, goal.value_2, dist);
    }
}

#[derive(Debug)]
pub struct Agent {
    pub sort_ability : SortAbility,
    pub index_affinity : Vec<u8>,
    pub age : u32,
    // ability to be influenced
    // ability to influence
    // observation
    // ability to be observed
    // tendency to act, observe, or influence (age?)
}

impl Agent {

    pub fn new<T : Rng>( rng : &mut T ) -> Self {
        let sort_ability = SortAbility { index_1 : rng.gen_range(0..255) 
                                       , index_2 : rng.gen_range(0..255) 
                                       , value_1 : rng.gen_range(0..255) 
                                       , value_2 : rng.gen_range(0..255) 
                                       };
        let index_affinity : Vec<u8> = (0..8).map(|i| i).collect(); 
        Agent { sort_ability
              , index_affinity
              , age : 0 
              }
    }

    pub fn act(&self) -> Action {

        Action::Act
    }
}
