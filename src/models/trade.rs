use super::item::Item;

pub struct Trade {
    pub player: String,
    pub item: Item,
    pub timeout_at_ms: u32,
}
