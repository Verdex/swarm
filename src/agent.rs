
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

    pub fn move_towards(&mut self, goal : &Self) {

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
