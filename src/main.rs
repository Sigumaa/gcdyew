#![deny(clippy::all)]
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default)]
struct GcdCalc {
    val1: Option<usize>,
    val2: Option<usize>,
    ans: Option<usize>,
}

enum Message {
    FieldVal1(String),
    FieldVal2(String),
    Run,
}

impl Component for GcdCalc {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("Initialization in progress ....");
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::FieldVal1(val1) => {
                self.val1 = val1.trim().parse().ok();
                log::info!("Processing value 1");
                false
            }
            Message::FieldVal2(val2) => {
                self.val2 = val2.trim().parse().ok();
                log::info!("Processing value 2");
                false
            }
            Message::Run => {
                if let Self {
                    val1: Some(mut val1),
                    val2: Some(mut val2),
                    ..
                } = self
                {
                    log::info!("Calculations are starting ....");
                    if val1 != 0 && val2 != 0 {
                        log::info!(" val1: {}, val2: {}", val1, val2);
                        let mut cnt = 1;
                        while val2 != 0 {
                            if val2 < val1 {
                                std::mem::swap(&mut val2, &mut val1);
                            }
                            val2 %= val1;
                            log::info!(
                                "{} calculation results. \n val1: {}, val2: {}",
                                cnt,
                                val1,
                                val2
                            );
                            cnt += 1;
                        }
                    }
                    log::info!("Calculations are done!");
                    self.ans = Some(val1);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div>
                <input type="text" oninput={ctx.link().callback(|e: InputEvent| Message::FieldVal1(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                <input type="text" oninput={ctx.link().callback(|e: InputEvent| Message::FieldVal2(e.target_unchecked_into::<HtmlInputElement>().value()))} />
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
    yew::start_app::<GcdCalc>();
}
