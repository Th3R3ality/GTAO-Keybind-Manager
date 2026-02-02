use iced::{
    Border,
    Element,
    Task,
    Theme,
    window,
    Length,
    widget::{
        column,
        container,
        pick_list,        
        row,
        button,
        space,
        text,
    },
};

use crate::{
    asset::{self, Icon},
    keybind_manager::{
        Message,
        State
    },
    screen::{
        About, Keybindings, Screen
    },
};

const HEADER_PADDING: f32 = 10.0;
const HEADER_BORDER_WIDTH: f32 = 0.0;
const HEADER_HEIGHT: f32 = 75.0;

pub fn run(state: State) -> iced::Result {
    let le_icon = window::icon::from_file_data(asset::ICON64, None).unwrap();
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
        Message::ScreenSelected(screen) => {
            state.screen = screen;
        },
        Message::Keybindings(message) => Keybindings::update(state, &message),
        Message::About(message) => About::update(state, &message),
    }

    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let header_navigation = container(row![
        container(
            button(
                text!("{}", Icon::Keyboard).size(40).font(asset::ICON_FONT).center()
            ).style(button::background)
            .on_press(Message::ScreenSelected(Screen::Keybindings(Keybindings::new())))
        ).align_right(iced::Fill),
        space().width(Length::Fixed(HEADER_PADDING)),
        container(
            button(
                text!("{}", Icon::Help).size(40).font(asset::ICON_FONT).center()
            ).style(button::background)
            .on_press(Message::ScreenSelected(Screen::About(About::new())))
        ),
    ]).align_right(Length::Shrink);
    
    let (content, screen_header) = match &state.screen {

        Screen::Keybindings(screen) => {
            let (a, b) = screen.view(&state);
            (a.map(Message::Keybindings), b.map(Message::Keybindings))
        },
        Screen::About(screen) => {
            let (a, b) = screen.view(&state);
            (a.map(Message::About), b.map(Message::About))
        },
        Screen::Landing => {(
            container(text("Welcome!").size(48).center().style(text::base)).center(iced::Fill).into(),
            space().into()
        )},
    };

    let header = container(row![
        container(screen_header).width(Length::Fill).height(Length::Fill),
        header_navigation,
    ])
    .height(Length::Fixed(HEADER_HEIGHT))
    .width(iced::Fill)
    .padding(HEADER_PADDING)
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();
        container::Style {
            border: Border { 
                width: HEADER_BORDER_WIDTH, 
                color: palette.background.weak.color,
                radius: 0.into(),
                },
            ..container::Style::default()
        }
    });

    column![
        header,
        space().height(Length::Fixed(1.0)).width(Length::Fill),
        container(content).width(Length::Fill).height(Length::Fill),
    ]
    .into()
}