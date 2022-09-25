pub mod components;

use components::CounterComponent::*;
use yew::prelude::*;

fn main() {
    println!("Hello, world!");
    yew::start_app::<CounterComponent>();
}
