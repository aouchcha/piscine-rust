pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub mod member;
pub use member::*;
pub mod boss;
pub use boss::*;
#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name : String,
    pub boss: Boss,
    pub members : HashMap<String,Member>,
    pub cities : HashSet<String>,
    pub wealth : u64,
}

impl Mob {
    pub fn recruit(&mut self, newmembwe :(&str, u32)) {
        self.members.insert(newmembwe.0.to_string(), Member { role: member::Role::Associate, age: newmembwe.1 });
    }
    pub fn attack(&mut self, other: &mut Mob) {
        let mut our_power : u32 = 0;
        let mut their_power : u32 = 0;
        let mut member_remove = String::new();
        let mut member_remove_age = 60;
        for (_member_name, member_data) in &self.members {
            match member_data.role {
                member::Role::Underboss => our_power += 4, 
                member::Role::Caporegime => our_power += 3, 
                member::Role::Soldier => our_power += 2, 
                member::Role::Associate => our_power += 1, 
            }
        }

        for (_member_name, member_data) in &other.members {
            match member_data.role {
                member::Role::Underboss => their_power += 4, 
                member::Role::Caporegime => their_power += 3, 
                member::Role::Soldier => their_power += 2, 
                member::Role::Associate => their_power += 1, 
            }
        }
        let loser: &mut Mob ;
        let winner: &mut Mob ;


        if their_power > our_power || their_power == our_power{
            loser =  self;
            winner = other;
        }else {
            loser =  other;
            winner = self;
        }
        for (member_name, member_data) in &loser.members {
            if member_data.age < member_remove_age {
                member_remove_age = member_data.age;
                member_remove = member_name.clone();
            } 
        }
        loser.members.remove(&member_remove);

        if loser.members.len() == 0 {
            winner.wealth += loser.wealth;
            loser.wealth = 0;
            for citie in &loser.cities{
                winner.cities.insert(citie.clone());
            }
            loser.cities.clear();
        }
    }
    pub fn steal(&mut self, target: &mut Mob, value_to_steal: u64) {
        println!("{:?}",self.wealth);
        println!("{:?}",target.wealth);
        println!("{:?}",value_to_steal);
        
        if (target.wealth as i32) - (value_to_steal as i32) <=  0 {
           self.wealth += target.wealth;
           target.wealth = 0;
           return
        }
        println!("{:?}",target.wealth - value_to_steal);
        self.wealth += value_to_steal;
        target.wealth -= value_to_steal;
    }

    pub fn conquer_city(&mut self, others: &[&Mob], citie: String) {
        for mob in others {
            for cities in &mob.cities {
                if citie == *cities {
                    return
                }
            }
        }
        self.cities.insert(citie);
    }
}
