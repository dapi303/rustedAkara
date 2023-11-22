use crate::{models::trade::Trade, trades_queue::TradesQueue};

pub struct Engine {
    current_trade: Option<Trade>,
    trades: TradesQueue,
}

impl Engine {
    pub fn new() -> Engine {
        return Self::default();
    }

    pub fn run(&mut self) {
        println!("run run run");
        self.trades.add_fake();
        self.trades.add_fake();
        self.trades.add_fake();
        self.trades.add_fake();
        self.trades.clean_timeouted();
    }

    //fn update_current(&self) {
    //if None == self.current_trade {
    //self.trades_queue.split_first
    //}
    //}
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            current_trade: None,
            trades: TradesQueue::new(),
        }
    }
}
