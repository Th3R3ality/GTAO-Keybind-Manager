use crate::input;
use crate::keycode;

use iced::wgpu::naga::back;
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

fn view(counter: &Counter) -> Element<Message> {
    let display = text(counter.value.to_string())
        .size(42)
        .style(|_| text::Style {
            color: Some(iced::Color::from_rgb8(220, 220, 255)),
            ..Default::default()
        }
    );

    let background: Element<Message> = scrollable(
        column![]
        .extend(keycode::keyboard::ALL.iter().map(|code| text((*code).1).into()))
        ,
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .into();

    let background2: Element<Message> = scrollable(
        column![]
        .extend(keycode::keyboard::ALL.iter().map(|code| text((*code).1).into()))
        ,
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .into();

    let background3: Element<Message> = scrollable(
        column![]
        .extend(keycode::keyboard::ALL.iter().map(|code| text((*code).1).into()))
        ,
    )
    .width(iced::Fill)
    .height(iced::Fill)
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

            text("lalala"),
            button("Reset")
                .on_press(Message::Reset)
                .padding([12, 32]),
        ]
        .padding(40)
        .spacing(32)
        .align_x(iced::Alignment::Center);

    stack!(row![background,background2,background3], main_content).into()
}