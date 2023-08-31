use gloo::console::{self, console, Timer};
use gloo::timers::callback::{Interval, Timeout};
use yew::{html, Component, Context, Html, InputEvent, SubmitEvent, MouseEvent, Properties, use_context, function_component};
use yew_hooks::use_session_storage;
use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/page")]
    AdditionalPage,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::AdditionalPage => html! { <h1>{ "Additional page" }</h1> },
    }
}

//
pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    UpdateTime,
    ToggleStuff,
    SetStuff(bool),
    UpdateInputVal(String)
}

//
pub struct App {
    time: String,
    messages: Vec<&'static str>,
    _standalone: (Interval, Interval),
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
    some_stuff: bool,
    input_val: String
}

impl App {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }

    fn cancel(&mut self) {
        self.timeout = None;
        self.interval = None;
    }
}
// #[derive(PartialEq)] ????????
// #[derive(Properties)]
// struct Props {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let standalone_handle =
            Interval::new(10, || console::debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: App::get_current_time(),
            messages: Vec::new(),
            _standalone: (standalone_handle, clock_handle),
            interval: None,
            timeout: None,
            console_timer: None,
            some_stuff: false,
            input_val: "".to_owned()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTimeout => {
                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(3, move || link.send_message(Msg::Done))
                };

                self.timeout = Some(handle);

                self.messages.clear();
                console::clear!();

                self.messages.push("Timer started!");
                self.console_timer = Some(Timer::new("Timer"));
                true
            }
            Msg::StartInterval => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);

                self.messages.clear();
                console::clear!();

                self.messages.push("Interval started!");
                true
            }
            Msg::Cancel => {
                self.cancel();
                self.messages.push("Canceled!");
                console::warn!("Canceled!");
                true
            }
            Msg::Done => {
                self.cancel();
                self.messages.push("Done!");

                // todo weblog
                // ConsoleService::group();
                console::info!("Done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }

                // todo weblog
                // ConsoleService::group_end();
                true
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                // todo weblog
                // ConsoleService::count_named("Tick");
                true
            }
            Msg::UpdateTime => {
                self.time = App::get_current_time();
                true
            }

            Msg::ToggleStuff => {
                self.some_stuff = !self.some_stuff;
                true
            },
            Msg::SetStuff( new_stuff ) => {
                console::info!("SetStuff: ", format!("{}", new_stuff));
                self.some_stuff = new_stuff;
                true
            },
            Msg::UpdateInputVal( new_input ) => {
                console::info!("UpdateInputVal: ", &new_input);
                self.input_val = new_input;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let has_job = self.timeout.is_some() || self.interval.is_some();
        let new_stuff = self.input_val.eq("1") || self.input_val.eq("true");

        let navigator = use_navigator();

        html! {
            <>
                <div id="buttons">
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartTimeout)}>
                        { "Start Timeout" }
                    </button>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartInterval)}>
                        { "Start Interval" }
                    </button>
                    <button disabled={!has_job} onclick={ctx.link().callback(|_| Msg::Cancel)}>
                        { "Cancel!" }
                    </button>

                    <input
                        type="text"
                        oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateInputVal(event.data().unwrap_or("".to_owned())))}
                    />
                    <button onclick={ctx.link().callback(move |_| Msg::SetStuff(new_stuff))}>
                        { "Set stuff" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::ToggleStuff)}>
                        { "Toggle stuff" }
                    </button>

                    <button onclick={navigator.push(&Route::Home)}>{ "Home" }</button>
                    <button onclick={navigator.push(&Route::AdditionalPage)}>{ "Additional page" }</button>
                </div>
                <div id="wrapper">
                    <div id="some_stuff">
                        { &self.some_stuff }
                    </div>
                    <div id="time">
                        { &self.time }
                    </div>
                    <div id="messages">
                        { for self.messages.iter().map(|message| html! { <p>{ message }</p> }) }
                    </div>
                </div>
                <div>
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}