use iced::{Element, Sandbox, Settings, Text};

struct OkspielMainView;

impl Sandbox for OkspielMainView {
    type Message = ();

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Okspiel".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Okspiel to handler your Ok full node wallets").into()
    }
}

fn main() -> iced::Result {
    OkspielMainView::run(Settings::default())
}
