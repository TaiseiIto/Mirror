use opencv::prelude::{MatTraitConst, MatTraitConstManual, VideoCaptureTrait};

enum Message {
    PressEnter,
    UpdateFrame,
}

struct State {
    camera: opencv::videoio::VideoCapture,
    image: Option<iced::widget::image::Handle>,
}

impl Default for State {
    fn default() -> Self {
        let camera: opencv::videoio::VideoCapture =
            opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();
        let image: Option<iced::widget::image::Handle> = None;
        Self { camera, image }
    }
}

fn boot() -> (State, iced::Task<Message>) {
    (
        State::default(),
        iced::Task::perform(async {}, |_| Message::UpdateFrame),
    )
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
            let State { camera, image } = state;
            let mut bgr = opencv::core::Mat::default();
            camera.read(&mut bgr).unwrap();
            let mut rgba = opencv::core::Mat::default();
            opencv::imgproc::cvt_color(&bgr, &mut rgba, opencv::imgproc::COLOR_BGR2RGBA, 0)
                .unwrap();
            *image = Some(iced::widget::image::Handle::from_rgba(
                rgba.cols() as u32,
                rgba.rows() as u32,
                rgba.data_bytes().unwrap().to_vec(),
            ));
            iced::Task::none()
        }
    }
}

fn view(state: &State) -> iced::Element<'_, Message> {
    if let State {
        camera: _,
        image: Some(image),
    } = state
    {
        let image: iced::widget::Image<iced::widget::image::Handle> = iced::widget::image(image)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .content_fit(iced::ContentFit::Contain);
        iced::widget::container(image)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .into()
    } else {
        iced::widget::column![].into()
    }
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
