#[derive(Debug, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate
}
#[derive(Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age : u32
}

impl Member {
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Underboss => panic!("Impossible to become a boss"),
            Role::Caporegime => self.role = Role::Underboss,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Associate => self.role = Role::Soldier
        }
    }
}