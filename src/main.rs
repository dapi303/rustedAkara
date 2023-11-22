use crate::engine::Engine;

mod chat;
mod engine;
mod models;
mod trades_queue;

fn main() {
    Engine::new().run();

    //let events = chat::get_events();
    //for event in events {
    //println!(
    //"-----> player {} item {} currency {:#?}",
    //event.player, event.item.name, event.price.currency
    //)
    //}
}
