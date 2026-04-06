#[derive(Clone, Debug, Default)]
struct Message();

#[derive(Clone, Debug, Default)]
struct State();

fn update(_: &mut State, _: Message) -> iced::Task<Message> {
    iced::Task::none()
}

fn view(_: &State) -> iced::Element<'_, Message> {
    iced::widget::column![].into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}
