use iced::{
    Element,
    widget::{
        space,
        container,
        column,
        text,
    },
};

use crate::keybind_manager::{
        State,
        VERSION,
    };

#[derive(Debug, Clone, PartialEq)]
pub struct About {

}
#[derive(Debug, Clone)]
pub struct Message {

}

impl About {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(_state: &State, _message: &Message) {

    }
    pub fn view(&self, _state: &State) -> (Element<'_, Message>,Element<'_, Message>) {
        (container(column![
            text!("Version {}", VERSION),
        ])
        .width(iced::Fill)
        .height(iced::Fill)
        .into(),
        space().into()
        )
    }
}