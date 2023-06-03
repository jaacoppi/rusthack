impl Stats {
    pub fn new() -> Stats {
        Stats {
            kills: 0
        }
    }
    pub fn add_kill(&mut self) {
        self.kills += 1;
    }
}

pub struct Stats {
    pub kills: i32,
}

