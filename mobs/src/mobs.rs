use std::collections::{HashMap, HashSet};

pub mod boss;
pub use crate::mobs::boss::*;
pub mod member;
pub use crate::mobs::member::*;

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name : String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64
}

impl Mob  {
    pub fn recruit(&mut self, member: (&str,u32)) {
        let _count = self.members.entry(member.0.to_owned()).or_insert(Member {
            role: Role::Associate,
            age: member.1
        });
    }
    pub fn attack(&mut self, m: &mut Mob) {
        let mut attacker_score = 0;
        let mut other_score = 0 ;
        let mut our_victime : String = String::new();
        let mut our_age :u32 = 60;
        let mut other_victime : String = String::new();
        let mut other_age :u32 = 60;
        let mut winner : &mut Mob;
        let mut loser : &mut Mob;

        for (name, member_data) in &self.members {
            match member_data.role {
                Role::Underboss => attacker_score += 4,
                Role::Caporegime => attacker_score += 3,
                Role::Soldier => attacker_score += 2,
                Role::Associate=> attacker_score += 1
            }
            if member_data.age < our_age {
                our_age = member_data.age;
                our_victime = name.clone();
            }
        }

        for (name, member_data) in &m.members {
            match member_data.role {
                Role::Underboss => other_score += 4,
                Role::Caporegime => other_score += 3,
                Role::Soldier => other_score += 2,
                Role::Associate=> other_score += 1
            }
            if member_data.age < other_age {
                other_age = member_data.age;
                other_victime = name.clone();
            }
        }

        if attacker_score > other_score {
            m.members.remove(&other_victime);
            winner = self;
            loser = m;
        }else {
            self.members.remove(&our_victime);
            winner = m;
            loser = self;
        }

        if loser.members.len() == 0 {
            for name in &loser.cities.clone() {
                winner.cities.insert(name.clone());
                loser.cities.remove(name);
            }
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }

    }

    pub fn steal(&mut self, m: &mut Mob, money: u64) {
        let mut ms: u64 = money;
        if m.wealth < money {
            ms = m.wealth;
        }
        self.wealth += ms;
        m.wealth -= ms;
    }

    pub fn conquer_city(&mut self, ms: &[&Mob], name: String) {
        let mut have_if = false;
        for mob in ms {
            if mob.cities.contains(&name) {
                have_if = true;
                break
            }
        }
        if !have_if {
            self.cities.insert(name);
        }
    }

}