use crate::models::{item::Item, trade::Trade};

pub struct TradesQueue {
    items: Vec<Trade>,
}

impl TradesQueue {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn clean_timeouted(&self) {
        for _item in self.items.iter() {
            println!("trying to clean");
        }
    }

    pub fn add_fake(&mut self) {
        self.items.push(Trade {
            player: "player1".to_string(),
            item: Item {
                name: "someItem1".to_string(),
                tab: "someTab1".to_string(),
                position: (2, 3),
            },
            timeout_at_ms: 3,
        });
    }
}

impl Default for TradesQueue {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
