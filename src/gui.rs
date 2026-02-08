use iced::{
    Border,
    Element,
    Task,
    Theme,
    window,
    Length,
    widget::{
        opaque,
        stack,
        column,
        container,
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

pub const HEADER_PADDING: f32 = 10.0;
pub const HEADER_BORDER_WIDTH: f32 = 0.0;
pub const HEADER_HEIGHT: f32 = 75.0;

pub const NAVIGATION_ICON_SIZE: f32 = 40.0;

#[derive(Debug, Clone)]
pub enum Prompt{
    UnsavedChanges(fn(&State) -> Element<'_, Message>),
    NewKeybind( fn(&State) -> Element<'_, Message>),
}

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
            if screen == state.screen {
                state.screen = Screen::Landing;
            }
            else {
                state.screen = screen;
            }
        },
        Message::Keybindings(message) => Keybindings::update(state, &message),
        Message::About(message) => About::update(state, &message),
    }

    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let header_navigation = container(row![
        navigation_button(state, Icon::Keyboard, Screen::Keybindings(Keybindings::new())),
        space().width(Length::Fixed(HEADER_PADDING)),
        navigation_button(state, Icon::Help, Screen::About(About::new())),
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
            column![
                container(text("Welcome!").size(48).center().style(text::base)).center(iced::Fill),
                container(space()).align_bottom(Length::Fixed(HEADER_HEIGHT)), // to make the text centered correctly
            ].into(),
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

    
    let prompt = container(opaque(
        match state.prompt {
            Some(Prompt::NewKeybind(fun)) => fun(state),
            Some(Prompt::UnsavedChanges(fun)) => fun(state),
            None => space().into(),
        }
    )).center(Length::Fill);
    stack![
        column![
            header,
            container(content).width(Length::Fill).height(Length::Fill),
        ],
        prompt,
    ]
    .into()
}

fn navigation_button(state: &State, icon: Icon, screen: Screen) -> Element<'static, Message> {
    container(
            button(
                asset::icon(icon).size(NAVIGATION_ICON_SIZE).center()
            ).style( if state.screen == screen { button::primary } else { button::background } )
            .on_press(Message::ScreenSelected(screen))
    ).into()
}