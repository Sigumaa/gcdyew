use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default)]
struct Gcd {
    n: Option<usize>,
    m: Option<usize>,
    ans: Option<usize>,
}

enum Message {
    Inputted1(String),
    Inputted2(String),
    Run,
}

impl Component for Gcd {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("Initialization in progress ....");
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Inputted1(n) => {
                self.n = n.trim().parse().ok();
                log::info!("Processing value n");
                false
            }
            Message::Inputted2(n) => {
                self.m = n.trim().parse().ok();
                log::info!("Processing value m");
                false
            }
            Message::Run => {
                if let Self {
                    n: Some(mut n),
                    m: Some(mut m),
                    ..
                } = self
                {
                    log::info!("Calculations are starting ....");
                    if n != 0 && m != 0 {
                        log::info!(" n: {}, m: {}", n, m);
                        let mut cnt = 1;
                        while m != 0 {
                            if m < n {
                                std::mem::swap(&mut m, &mut n);
                            }
                            m %= n;
                            log::info!("{} calculation results. \n n: {}, m: {}", cnt, n, m);
                            cnt += 1;
                        }
                    }
                    log::info!("Calculations are done!");
                    self.ans = Some(n);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div>
                <input type="text" oninput={ctx.link().callback(|e: InputEvent| Message::Inputted1(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                <input type="text" oninput={ctx.link().callback(|e: InputEvent| Message::Inputted2(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                <button onclick={ctx.link().callback(|_| Message::Run)}>{"Run"}</button>
            </div>
            <div>
                if let Some(ans) = self.ans {
                    <b>{ans}</b>
                }
            </div>
            <div>
            <h2>{"二つの数字のGCDを求めます．"}</h2>
            <h2>{"現状簡易版です．CSS等一切当てていません．今後(おそらくたぶんきっと)改善予定です．"}</h2>
            </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Gcd>();
}
