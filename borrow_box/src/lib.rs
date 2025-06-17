#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(
            GameSession {
                id: id,
                p1: (p1_name.clone(), 0 as u16),
                p2: (p2_name.clone(), 0 as u16),
                nb_games: nb_games,
            }
        )
    }
    pub fn read_winner(&self) -> (String, u16) {
        let mut max = 0;
        println!("{}",self.nb_games);
        println!("{}",(self.nb_games/2) +1);

        if self.p1.1 >= (self.nb_games/2) +1 {
            return (self.p1.0.clone(), self.p1.1)
        }else if self.p2.1 >= (self.nb_games/2) +1 {
            return (self.p2.0.clone(), self.p2.1)
        }else {
            if self.p1.1 > self.p2.1 {
                return (self.p1.0.clone(), self.p1.1)
            }else if self.p1.1 < self.p2.1 {
                return (self.p2.0.clone(), self.p2.1)
            }else {
                if self.p1.1 > self.p2.1 {
                    max = self.p1.1 ;
                }else {
                    max = self.p2.1 
                }
                return ("Same score! tied".to_string(),max)
            }
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.p1.1 >= (self.nb_games/2) +1 || self.p2.1 >= (self.nb_games/2) +1 {
            return
        }
        if self.p1.0 == user_name {
            self.p1.1 += 1;
        }else if self.p2.0 == user_name{
            self.p2.1 += 1;
        }
    }
    pub fn delete(self) -> String {
        let end = self;
        format!("game deleted: id -> {}",end.id)
    }
}
