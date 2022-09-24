use yew::prelude::*;

enum Msg {
    AddOne,
    MinusOne
}

struct CounterComponent {
    count: i32,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // Re-render
            }
            Msg::MinusOne => {
                self.count -= 1;
                true // Re-render
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="text-center border border-black h-screen flex flex-col justify-center items-center bg-gradient-to-l from-blue-400 to-indigo-800">
                <p1 class="text-6xl font-extrabold mb-2 text-transparent bg-clip-text bg-gradient-to-r from-red-400 to-indigo-800">{"Welcome to yew!"}</p1>
                <p2 class="text-xl mb-4 text-white">{"Built with love using WASM + Rust"}</p2>

                <div class="border rounded p-4 bg-gradient-to-r from-red-400 to-indigo-800 w-11/12 hover:translate-y-2 transition-all cursor-pointer">
                    <p class="text-bold text-3xl text-white">{ self.count }</p>
                    <button class="border border-white text-white rounded p-2 m-2" onclick={link.callback(|_| Msg::AddOne)}>{ "Add one" }</button>
                    <button class="border border-white text-white rounded p-2 m-2" onclick={link.callback(|_| Msg::MinusOne)}>{ "Minus one" }</button>
                </div>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
