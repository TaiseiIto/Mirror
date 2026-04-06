#[derive(Default)]
enum Message {
    #[default]
    PressEnter,
}

struct State {
    camera: opencv::videoio::VideoCapture,
}

impl Default for State {
    fn default() -> Self {
        Self {
            camera: opencv::videoio::VideoCapture::default().unwrap(),
        }
    }
}

fn boot() -> (State, iced::Task<Message>) {
    (State::default(), iced::Task::none())
}

fn subscription(_: &State) -> iced::Subscription<Message> {
    iced::event::listen_with(|event, _, _| match event {
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
            match key.as_ref() {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                    Some(Message::PressEnter)
                }
                _ => None,
            }
        }
        _ => None,
    })
}

fn update(_: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::PressEnter => iced::exit(),
    }
}

fn view(_: &State) -> iced::Element<'_, Message> {
    iced::widget::column![].into()
}

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .subscription(subscription)
        .window(iced::window::Settings {
            fullscreen: true,
            ..iced::window::Settings::default()
        })
        .run()
}
