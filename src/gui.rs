use iced::{
    Border,
    Element,
    Task,
    Theme,
    window,
    widget::{
        column,
        container,
        pick_list,        
        row,
        button,
        scrollable,
        text,
    },
};

use crate::{
    asset,
    keybind_manager::{
        Message,
        State
    },
    screen::{
        Screen, about, keybinding
    }
};

pub fn run(state: State) -> iced::Result {
    let le_icon = window::icon::from_file_data(asset::ICON32, None).unwrap();
    iced::application( move || (state.clone(), iced::Task::none()), update, view)
        .window(window::Settings {
            icon: Some(le_icon),
            ..Default::default()
        })
        .title(State::title)
        .theme(Theme::TokyoNight)
        .antialiasing(true)
        .font(asset::ICON_FONT_DATA)
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ProfileSelected(mut profile) => {
            let res = profile.load_xml().err();
            match res {
                Some(err) => println!("Error: {} | {}", err.0, err.1),
                None => {
                    if let Err(err) = profile.write_xml(){
                        println!("Error writing xml '{}': {} | {}", profile.name, err.0, err.1);
                    }
                    state.current_profile = Some(profile);
                },
            }
        },
        Message::ScreenSelected(screen) => {
            state.screen = screen;
        },
        Message::KeybindingMessage(message) => keybinding::Screen::update(state, &message),
        Message::AboutMessage(message) => about::Screen::update(state, &message),
    }

    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let header = container(row![
    container(
        row![
            text("Profile: ")
                .size(28)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Shrink).height(iced::Fill),
            container(
            pick_list(state.available_profiles.clone(), state.current_profile.clone(), Message::ProfileSelected)
                .placeholder("none")
                .text_size(28)
            )
            .height(iced::Fill)
            .align_y(iced::Center),
        ]
    ).align_left(iced::Fill),
    container(
        button(
            text!("{}", "\u{e8fd}").size(40).font(asset::ICON_FONT).center()
        ).on_press(Message::ScreenSelected(Screen::About(about::Screen::new())))
    ).align_right(iced::Fill),

    ])
    .width(iced::Fill)
    .padding(10)
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();
        container::Style {
            border: Border { 
                width: 2.0, 
                color: palette.background.weak.color,
                radius: 0.into(),
                },
            ..container::Style::default()
        }
    });
    
    let content: Element<Message> = scrollable( match &state.screen {
        Screen::Keybinding(screen) => screen.view(&state).map(Message::KeybindingMessage),
        Screen::About(screen) => screen.view(&state).map(Message::AboutMessage),
    })
    .width(iced::Fill)
    .height(iced::Fill)
    .into();

    column![
        container(header).height(iced::FillPortion(9)),
        container(content).height(iced::FillPortion(90)),
    ]
    .into()
}