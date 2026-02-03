use iced::{
    Element, Length, widget::{
        column, container, space, text
    }
};

use crate::{gui, keybind_manager::{
        State,
        VERSION,
    }};

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
        (column![
            container(column![
                text!("Tool developed by Reality"),
                text!("Version {}", VERSION).size(28),
            ])
            .center(Length::Fill)
            .padding(20),
            container(space()).align_bottom(Length::Fixed(gui::HEADER_HEIGHT)), 
        ].into(),
        space().into()
        )
    }
}