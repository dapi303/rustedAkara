pub struct Item {
    name: String,
    price: String,
    tab: String,
    row: u32,
    col: u32,
}

pub enum Event {
    Request {
        player: String,
        item: Item,
        time_stamp: u32,
    },
    AreaEnter(String),
    AreaLeave(String),
    OpenTrade,
    CloseTrade,
}
