//! This is a simple implementation of a web interface for the API.

use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo_net::http::Request;

/// This will return the API endpoint, which can be set via an environment
/// variable, defaulting to same host, same port, "/braille".
fn get_endpoint() -> &'static str {
    return option_env!("AA_ENDPOINT").unwrap_or("/braille");
}

/// This enum entails the states the Braille display can be in.
#[derive(PartialEq, Eq, Clone)]
enum BrailleState {
    /// Waiting for an URL, nothing to show.
    Waiting,
    /// Request ongoing.
    Requesting,
    /// Showing a braille string!
    Showing(AttrValue),
    /// Got an error.
    Error(AttrValue)
}

/// This component displays the API result.
#[derive(PartialEq, Eq, Properties, Clone)]
struct BrailleProps {
    state: BrailleState
}

/// This displays the current braille string.
struct BrailleDisplay {}

impl Component for BrailleDisplay {
    type Message = ();
    type Properties = BrailleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {};
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return match ctx.props().state {
            BrailleState::Waiting => html! {
                <div class="waiting"></div>
            },
            BrailleState::Requesting => html! {
                <div class="waiting">{ "Loading..." }</div>
            },
            BrailleState::Showing(ref s) => html! {
                <div class="braille">{ s }</div>
            },
            BrailleState::Error(ref s) => html! {
                <div class="error">{ s }</div>
            },
        }
    }
}

/// This is the main app component.
struct App {
    /// The URL currently in the form.
    url: String,
    /// The state for the Braille component.
    state: BrailleState
}

/// This entails the messages the app can send to itself.
enum AppMsg {
    /// A change in the URL in the form.
    UrlChange(String),
    /// Generate button hit.
    GenBraille,
    /// Set the BrailleDisplay state.
    SetBrailleState(BrailleState)
}

async fn do_request(img_url: String) -> AppMsg {
    let params = [
        ("img_url", &img_url)
    ];
    let req = Request::get(get_endpoint())
        .query(params)
        .send()
        .await;
    let bs: BrailleState = match req {
        Ok(resp) => {
            if resp.ok() {
                match resp.text().await {
                    Ok(s) => BrailleState::Showing(s.into()),
                    Err(e) => BrailleState::Error(e.to_string().into())
                }
            } else {
                match resp.text().await {
                    Ok(s) => BrailleState::Error(s.into()),
                    Err(e) => BrailleState::Error(e.to_string().into())
                }
            }
        },
        Err(err) => BrailleState::Error(err.to_string().into()),
    };
    return AppMsg::SetBrailleState(bs);
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            url: "".to_owned(),
            state: BrailleState::Waiting
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UrlChange(s) => self.url = s,
            AppMsg::GenBraille => {
                self.state = BrailleState::Requesting;
                ctx.link().send_future(do_request(self.url.clone()))
            },
            AppMsg::SetBrailleState(bs) => self.state = bs,
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let url_cb = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            return Self::Message::UrlChange(input.value());
        });
        let btn_cb = ctx.link().callback(|_e: MouseEvent| {
            return Self::Message::GenBraille;
        });
        return html! {
            <>
                <h3>{ "Image to Braille" }</h3>
                <br />
                <br />
                <input oninput={url_cb} type="text" />
                <br />
                <button onclick={btn_cb}>{ "Go" }</button>
                <br />
                <br />
                <BrailleDisplay state={self.state.clone()} />
            </>
        }
    }


}

fn main() {
    yew::Renderer::<App>::new().render();
}
