#[derive(Default)]
struct Counter {
    value: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

use iced::padding;
use iced::widget::{button, column, text, Column, container};
impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value).size(50),
            container("Independent padding!")
                .padding(padding::vertical(30).left(20).right(80))
                .style(container::bordered_box),
            

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    iced::run(Counter::update, Counter::view);
}