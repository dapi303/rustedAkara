use super::item::Item;

#[derive(Debug, PartialEq)]
pub enum Currency {
    Exalt,
    Chaos,
    Alch,
}

pub struct Price {
    pub quantity: u32,
    pub currency: Currency,
}

pub struct ChatEvent {
    pub player: String,
    pub item: Item,
    pub price: Price,
}

impl Currency {
    pub fn new(s: &str) -> Result<Self, ()> {
        match s {
            "alch" => Ok(Currency::Alch),
            "exalted orb" => Ok(Currency::Exalt),
            "Chaos" => Ok(Currency::Chaos),

            _ => Err(()),
        }
    }
}
