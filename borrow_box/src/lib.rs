use std::cmp::Ordering;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}
impl GameSession {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id: i,
            p1: (pl1, 0),
            p2: (pl2, 0),
            nb_games: n,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        match self.p1.1.cmp(&self.p2.1) {
            Ordering::Greater => self.p1.clone(),
            Ordering::Equal => ("Same score! tied".to_string(), self.p1.1),
            Ordering::Less => self.p2.clone(),
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.is_ended() {
            return
        }
        let mut quick_fix = ("Same score! tied".to_string(),  self.p1.1);
        let p = match user_name {
            name if self.p1.0 == name => &mut self.p1,
            name if self.p2.0 == name => &mut self.p2,
            _ => &mut quick_fix //panic!("Invalid user_name")
        };
        p.1 += 1;
    }
    fn is_ended(&self) -> bool {
        self.p1.1 + self.p2.1 >= self.nb_games
            || self.p1.1 > self.nb_games / 2
            || self.p2.1 > self.nb_games / 2
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}