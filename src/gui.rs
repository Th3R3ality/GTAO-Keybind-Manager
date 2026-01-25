use crate::input;

use iced::{
    Element, Size, Task, Theme, alignment, widget::{
        button, column, container, row, scrollable, stack, text
    }
};

pub fn run() -> iced::Result {
    iced::application(|| (Counter::default(), Task::none()), update, view)
        .title(Counter::title)
        .theme(Theme::Dark)
        .antialiasing(true)
        .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn title(&self) -> String {
        format!("Counter: {}", self.value)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

fn update(counter: &mut Counter, message: Message) -> Task<Message> {
    match message {
        Message::Increment => {
            counter.value += 1;
        }
        Message::Decrement => {
            counter.value -= 1;
        }
        Message::Reset => {
            counter.value = 0;
        }
    }
    Task::none()
}

static mut bloopy: usize = 0;
fn view(counter: &Counter) -> Element<Message> {
    let display = text(counter.value.to_string())
        .size(42)
        .style(|_| text::Style {
            color: Some(iced::Color::from_rgb8(220, 220, 255)),
            ..Default::default()
        }
    );

    let control: &str;

    unsafe {
        control = input::ALL.get(bloopy)
        .copied()
        .unwrap_or("FUCK");
        
        bloopy += 1; bloopy %= input::ALL.len();
    }

    

    let background: Element<Message> = scrollable(
        column![].extend(input::ALL.iter().map(|code| text(*code).into())),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .into();

    let main_content =
        column![
            text("Counter Example")
                .size(32)
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),

            row![
                button("-10").on_press(Message::Decrement),
                button("-1") .on_press(Message::Decrement),
                display,
                button("+1") .on_press(Message::Increment),
                button("+10").on_press(Message::Increment),
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center),

            text(control),
            button("Reset")
                .on_press(Message::Reset)
                .padding([12, 32]),
        ]
        .padding(40)
        .spacing(32)
        .align_x(iced::Alignment::Center);

    stack!(background, main_content).into()
}