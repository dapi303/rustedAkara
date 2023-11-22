use crate::models::{
    chat_event::{ChatEvent, Currency, Price},
    item::Item,
};
use regex::Regex;

//@From Player Hi, I would like to buy your Pandemonium Peak Tricorne listed for 1 alch in Ancestor (stash tab "~b/o 1 alch"; position: left 3, top 3)
fn get_messages() -> Vec<String> {
    return vec![
"@From Player1 Hi, I would like to buy your Pandemonium Peak Tricorne listed for 1 alch in Ancestor (stash tab \"~b/o 1 alch\"; position: left 3, top 3)".to_string(),
"SOme spam".to_string(),
"@From Player2 Hi, I would like to buy your Pandemonium Peak X listed for 4 Chaos in Ancestor (stash tab \"$$$\"; position: left 2, top 1)".to_string(),
"@From Player2 Hi, I would like to buy your Pandemonium Peak X listed for 4 Chaos in Ancestor (stash tab some_tab; position: left 2, top 1)".to_string(),
"@From Player2 blah blah...".to_string(),
"SOme spam".to_string(),
    ];
}

//fn get_str_or_empty

pub fn convert_to_event(message: &String) -> Result<ChatEvent, ()> {
    let message_regex = Regex::new(r#"^@From (\w+) Hi, I would like to buy your (.*) listed for (\d+) (\w+) in (\w+) \(stash tab (.*)\;.*position: left (\d+), top (\d+)\)$"#).unwrap();
    let found = message_regex.captures(&message);

    if found.is_some() {
        let result = found.unwrap();
        if result.len() > 7 {
            let player = &result[1];
            //let player = result.get(1).map_or("", |x| x.as_str());
            let item_name = result.get(2).map_or("", |x| x.as_str());
            let quantity = result
                .get(3)
                .map_or(0, |x| x.as_str().parse::<u32>().unwrap());
            let currency_str = result.get(4).map_or("", |x| x.as_str());
            let currency = Currency::new(currency_str);
            if currency.is_err() {
                return Err(());
            }
            //let league = result.get(5).map_or("", |x| x.as_str());
            let tab = result.get(6).map_or("", |x| x.as_str());
            let col = result
                .get(7)
                .map_or(0, |x| x.as_str().parse::<u32>().unwrap());
            let row = result
                .get(8)
                .map_or(0, |x| x.as_str().parse::<u32>().unwrap());

            let event = ChatEvent {
                player: player.to_owned(),
                item: Item {
                    name: item_name.to_owned(),
                    tab: tab.to_owned(),
                    position: (col, row),
                },
                price: Price {
                    quantity,
                    currency: currency.unwrap(),
                },
            };
            return Ok(event);
        }
    }
    return Err(());
}

pub fn get_events() -> Vec<ChatEvent> {
    let messages = get_messages();

    let mut events = Vec::new();
    for message in messages {
        let event = convert_to_event(&message);
        if event.is_ok() {
            events.push(event.unwrap());
        }
    }

    return events;
}

#[cfg(test)]
mod tests {
    use crate::chat::convert_to_event;

    const VALID_MESSAGE: &str = "@From Player1 Hi, I would like to buy your Pandemonium Peak Tricorne listed for 1 alch in Ancestor (stash tab \"~b/o 1 alch\"; position: left 2, top 3)";

    #[test]
    fn valid_offer_properly_converted() {
        let event = convert_to_event(&VALID_MESSAGE.to_string()).unwrap();
        assert_eq!(event.player, "Player1");
        assert_eq!(event.item.name, "Pandemonium Peak Tricorne");
        assert_eq!(event.item.tab, "\"~b/o 1 alch\"");
        assert_eq!(event.price.quantity, 1);
        //assert_eq!(event.price.currency, Currency::Alch);
        assert_eq!(event.item.position, (2, 3));
    }

    #[test]
    fn special_characters_in_player_name() {
        let message = "@From PlayerÓÓWZŻ1 Hi, I would like to buy your Pandemonium Peak Tricorne listed for 1 alch in Ancestor (stash tab \"~b/o 1 alch\"; position: left 3, top 3)";
        let event = convert_to_event(&message.to_string()).unwrap();
        assert_eq!(event.player, "PlayerÓÓWZŻ1");
    }

    #[test]
    fn invalid_offer_ignored() {
        let message = "@From Player1 blah blabh bklajsdkla sda";
        let event = convert_to_event(&message.to_string());
        assert!(!event.is_ok());
    }

    #[test]
    fn offer_with_extra_text_at_end_ignored() {
        let message = format!("{} some extra stuff", VALID_MESSAGE);
        let event = convert_to_event(&message.to_string());
        assert!(!event.is_ok());
    }
    #[test]
    fn offer_with_extra_text_at_start_ignored() {
        let message = format!("some extra stuff {}", VALID_MESSAGE);
        let event = convert_to_event(&message.to_string());
        assert!(!event.is_ok());
    }
}
