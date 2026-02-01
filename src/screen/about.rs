use iced::{
    Element,
    widget::{
        container,
        column,
        text,
    },
};

use crate::keybind_manager::{
        State,
        VERSION,
    };

#[derive(Debug, Clone)]
pub struct Screen {

}
#[derive(Debug, Clone)]
pub struct Message {

}

impl Screen {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(_state: &State, _message: &Message) {

    }
    pub fn view(&self, _state: &State) -> Element<'_, Message> {
        container(column![
            text!("Version {}", VERSION),
        ])
        .width(iced::Fill)
        .height(iced::Fill)
        .into()


    }
}