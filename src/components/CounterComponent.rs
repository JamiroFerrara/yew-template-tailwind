use yew::prelude::*;

pub enum Msg {
    AddOne,
    MinusOne
}

pub struct CounterComponent {
    count: i32,
    title: String
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0, title: "Built with love using WASM + Rust + Tailwind".to_string()}
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
            <div class="text-center border border-black h-screen flex flex-col justify-center items-center bg-gray-800">
                <p1 class="text-6xl font-extrabold mb-2 text-transparent bg-clip-text bg-gradient-to-r from-red-400 to-indigo-800">{"Welcome to yew!"}</p1>
                <p2 class="text-xl mb-4 text-white">{&self.title}</p2>

                <div>
                    <p class="text-bold text-3xl text-white">{ self.count }</p>
                    <button class="border border-white text-white rounded p-2 m-2" onclick={link.callback(|_| Msg::AddOne)}>{ "Add one" }</button>
                    <button class="border border-white text-white rounded p-2 m-2" onclick={link.callback(|_| Msg::MinusOne)}>{ "Minus one" }</button>
                </div>

            </div>
        }
    }
}
