#[derive(Clone, Debug, Default)]
struct Message();

#[derive(Clone, Debug, Default)]
struct State();

fn boot() -> (State, iced::Task<Message>) {
    (State::default(), iced::Task::none())
}

fn update(_: &mut State, _: Message) -> iced::Task<Message> {
    iced::Task::none()
}

fn view(_: &State) -> iced::Element<'_, Message> {
    iced::widget::column![].into()
}

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .window(iced::window::Settings {
            fullscreen: true,
            ..iced::window::Settings::default()
        })
        .run()
}
