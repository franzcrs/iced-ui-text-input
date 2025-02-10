use iced::alignment::Horizontal;
use iced::widget::{column, text_input}; // add import for text_input
use iced::window::{Settings, Position, Level};
use iced::font::{Family, Weight};
use iced::{Color, Element, Font, Length, Padding, Size};
use iced::widget::text_input::Style; // add import for style sheet
use iced::{Background, Border}; // add import for Background and Border
use iced::border::Radius; // add import for Radius

struct State {
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
}

fn view(state: &State) -> Element<'_, Message> {
    let input = // Comment to only display the text_input component
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
        .style(|_theme, status| { // replaced custom type with a closure
            match status {
                iced::widget::text_input::Status::Active 
                | iced::widget::text_input::Status::Hovered => Style {
                    background: Background::Color(Color::from_rgb(0.1490, 0.1490, 0.1333)),//38, 38, 34
                    border: Border {
                        color: Color::from_rgb(0.2470, 0.2431, 0.2313), // 63, 62, 59
                        width: 1.0,
                        radius: Radius::new(2.0),
                    },
                    placeholder: Color::from_rgb(0.3804, 0.3764, 0.3725),// 97, 96, 95
                    value: Color::WHITE,
                    selection: Color::from_rgb(0.2784, 0.3843, 0.5294), // 71, 98, 135
                    icon: Color::BLACK
                },
                iced::widget::text_input::Status::Focused => Style {
                    background: Background::Color(Color::from_rgb(0.1490, 0.1490, 0.1333)),//38, 38, 34
                    border: Border {
                        color: Color::from_rgb(0.2196, 0.3922, 0.5412), // 56, 100, 138
                        width: 3.5,
                        radius: Radius::new(2.0),
                    },
                    placeholder: Color::from_rgb(0.3804, 0.3764, 0.3725),// 97, 96, 95
                    value: Color::WHITE,
                    selection: Color::from_rgb(0.2784, 0.3843, 0.5294), // 71, 98, 135
                    icon: Color::BLACK
                },
                iced::widget::text_input::Status::Disabled => Style {
                    background: Background::Color(Color::from_rgb(0.1490, 0.1490, 0.1333)),//38, 38, 34
                    border: Border {
                        color: Color::from_rgb(0.2470, 0.2431, 0.2313), // 63, 62, 59
                        width: 1.0,
                        radius: Radius::new(2.0),
                    },
                    placeholder: Color::from_rgb(0.5, 0.5, 0.5),
                    value: Color::from_rgb(0.5, 0.5, 0.5),
                    selection: Color::from_rgb(0.2745, 0.2745, 0.2745), // 70, 70, 70
                    icon: Color::BLACK
                },
            }
        })
        .align_x(Horizontal::Left)
        .font(Font {
            family: Family::SansSerif,
            weight: Weight::Medium,
            ..Default::default()
        })
        .size(13)
        .width(Length::Fill)
        .line_height(1.0)
        .padding(Padding::from([5, 7]))
        // .into() // Uncomment to only display the text_input component
        ;   // Comment to only display the text_input component
    column![input]   // Comment to only display the text_input component
        .padding(20)   // Comment to only display the text_input component
        .into()   // Comment to only display the text_input component
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            content: String::new(),
        }
    }
}

fn main() -> iced::Result {
  // iced::run("", update, view) // You can either use this or the method below
  iced::application("", update, view)
  .window(Settings {
    size: Size::new(400.0, 200.0),
    position: Position::default(),
    min_size: None,
    max_size: None,
    visible: true,
    resizable: false,
    decorations: true,
    transparent: true,
    level: Level::default(),
    icon: None,
    exit_on_close_request: true,
    ..Default::default()
  })
  .run()
}
