use opencv::prelude::{
    MatTraitConst,
    MatTraitConstManual,
    VideoCaptureTrait,
}; 

enum Message {
    PressEnter,
    UpdateFrame,
}

struct State {
    camera: opencv::videoio::VideoCapture,
    frame: opencv::core::Mat,
}

impl Default for State {
    fn default() -> Self {
        let camera: opencv::videoio::VideoCapture =
            opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();
        let frame: opencv::core::Mat = opencv::core::Mat::default();
        Self { camera, frame }
    }
}

fn boot() -> (State, iced::Task<Message>) {
    (State::default(), iced::Task::perform(async {()}, |_| Message::UpdateFrame))
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

fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::PressEnter => iced::exit(),
        Message::UpdateFrame => {
            let State {
                camera,
                frame,
            } = state;
            camera.read(frame).unwrap();
            iced::Task::none()
        },
    }
}

fn view(state: &State) -> iced::Element<'_, Message> {
    let State {
        camera,
        frame,
    } = state;
    let image: iced::widget::image::Handle = iced::widget::image::Handle::from_rgba(frame.cols() as u32, frame.rows() as u32, frame.data_bytes().unwrap().to_vec());
    let image: iced::widget::Image<iced::widget::image::Handle> = iced::widget::image(image);
    iced::widget::container(image)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
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
