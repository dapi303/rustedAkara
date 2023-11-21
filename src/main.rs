mod chat;
mod models;

fn main() {
    println!("Hello, world!");

    let events = chat::get_events();
    for event in events {
        println!(
            "-----> player {} item {} currency {:#?}",
            event.player, event.item.name, event.price.currency
        )
    }
}
