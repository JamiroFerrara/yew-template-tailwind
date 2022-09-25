pub mod components;

use components::CounterComponent::*;
use yew::prelude::*;

fn main() {
    yew::start_app::<CounterComponent>();
}
