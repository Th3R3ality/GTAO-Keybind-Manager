use iced::{
    Element,
    FillPortion,
    widget::{
        column,
        container,
        row,
        space,
        text
    }
};

use crate::keycode;
use crate::input;
use crate::keybind_manager::{
    State,
};


#[derive(Debug, Clone)]
pub struct Screen {

}

#[derive(Debug, Clone)]
pub enum Message {

}
impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn update(_state: &mut State, _message: &Message) {
        
    }

    pub fn view(&self, state: &State) -> Element<'_, Message> {
        let mut background_column: iced::widget::Column<'_, Message> = column![];
        if let Some(profile) = &state.current_profile {
            if let Some(keybinds) = &profile.keybinds {
                let mut keybinds_sorted = keybinds.clone();
                keybinds_sorted.sort_by_key(|keybind|
                    (keybind.1, keybind.0)
                );
                background_column = background_column.extend(
                    keybinds_sorted.iter().enumerate().map(|(index, keybind)|{

                        let (input, category, keycode) =
                            (input::from_index(keybind.0),
                            keycode::category_from_index(keybind.1),
                            keycode::keycode_from_indexes(keybind.1, keybind.2)
                        );
                        
                        container(row![
                            space().width(FillPortion(1)),
                            container(column![
                                    text(input).size(20),
                                    text!("#{}", keybind.3).size(12).style(text::warning),
                            ]).width(iced::FillPortion(20))
                            ,container(column![
                                text(category.1).size(20),
                                text(category.2).size(12).style(text::primary),
                            ]).width(iced::FillPortion(15))
                            ,container(column![
                                text(keycode.2).size(20),
                                text(keycode.1).size(12).style(text::primary),
                            ]).width(iced::FillPortion(10))
                            ,
                        ])
                        .width(iced::Fill)
                        .style( if index % 2 == 0 {container::transparent} else {container::dark} )
                        //.height(iced::Length::Fixed(75.0))
                        .padding(4)
                        .into()
                    })
                );
            }
        }
        return background_column.into()
    }
}