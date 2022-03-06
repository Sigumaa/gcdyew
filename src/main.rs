use std::str::FromStr;
use yew::prelude::*;
use web_sys::HtmlInputElement;
struct Gcd {
    ans: usize,
    intex: String,
}

enum Message {
    Inputted(String),
    Run,
}

impl Component for Gcd {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            intex: "1x1".to_string(),
            ans: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Inputted(n)=>{
                self.intex=match n.trim().parse(){
                    Ok(ok)=>ok,
                    Err(_)=>String::default(),
                };
                false
            }
            Message::Run => {
                let intext = parse_pair(&self.intex, 'x').expect("parse error");
                let mut n = intext.0;
                let mut m = intext.1;
                log::info!("intext.0 {}",intext.0);
                log::info!("intext.1 {}",intext.1);
                if n != 0 && m != 0 {
                    while m != 0 {
                        if m < n {
                            std::mem::swap(&mut m, &mut n);
                        }
                        m %= n;
                        log::info!("n:{}",n);
                        log::info!("m:{}",m);
                    }
                }
                self.ans = n;
                log::info!("ans:{}",n);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div>
                <input type="text" oninput={ctx.link().callback(|e: InputEvent| Message::Inputted(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                <button onclick={ctx.link().callback(|_| Message::Run)}>{"Run"}</button>
            </div>
            <div>
                {self.ans}
            </div>
            <div>
            <h2>{"二つの数字のGCDを求めます．"}</h2>
            <h2>{"現状簡易版です．CSS等一切当てていません．今後(おそらくたぶんきっと)改善予定です．"}</h2>
            <h3>{"N,Mを任意の正の整数とします．  `NxM`  二つの数字をxで区切って入力してください．"}</h3>
            </div>
            </div>
        }
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn main() {
    yew::start_app::<Gcd>();
    wasm_logger::init(wasm_logger::Config::default());
}
