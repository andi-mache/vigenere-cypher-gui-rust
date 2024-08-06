use iced::{alignment,executor, widget::{column,row,button,container, progress_bar, text, text_input, Button}, window, Application, Command, Font, Length, Settings};
use iced::widget::text_input::{Icon,Side};
use iced::theme::{self, Theme};
use iced::Alignment::Center;

mod vigenere;

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings { 
            size: iced::Size::new(300.0, 500.0), 
            resizable: (false),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct App{
    //slider_value: f32,
    encrypt_button: button::State,
    decrypt_button: button::State,
    input_text: String,
    key_phrase: String,
    output_text: String,
}


#[derive(Debug, Clone)]
enum Message {
    Encrypt,
    Decrypt,
 //   EditMessage(String),
 //   EditKey(String),
    UpdateInputText(String),
    UpdateKeyPhrase(String),
}



impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                encrypt_button: button::State::new(),
                decrypt_button: button::State::new(),
                input_text: String::new(),
                key_phrase: String::new(),
                output_text: String::new(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Vigenère Cipher GUI")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Encrypt => {
                let encrypted = vigenere::encrypt(&self.input_text, &self.key_phrase);
                self.output_text = encrypted;
            }
            Message::Decrypt => {
                let decrypted = vigenere::decrypt(&self.input_text, &self.key_phrase);
                self.output_text = decrypted;
            }
            Message::UpdateInputText(new_text) => {
                self.input_text = new_text;
            }
            Message::UpdateKeyPhrase(new_key) => {
                self.key_phrase = new_key;
            }
        }
        Command::none()
    }
    
    fn view(&self) -> iced::Element<Message> {


        let controls = row![
            Button::new("encrypt")
            .width(Length::Fill)
            .padding(10)
            .style(theme::Button::Primary)
            .on_press(Message::Encrypt),
            Button::new("decrypt")
            .width(Length::Fill)
            .padding(10)
            .style(theme::Button::Primary)
            .on_press(Message::Decrypt)
        ]
        .spacing(10);


        let content = column![
            text_input("Put your KEY here", &self.key_phrase)
            .on_input(|s| Message::UpdateKeyPhrase(s))
            .padding(10)
            .size(20)
            .icon(Icon {
                font: Font::DEFAULT,
                code_point: '\u{2705}',
                size: None,
                spacing: 10.,
                side: Side::Left,
                }),
            text_input("Put your MESSAGE here", &self.input_text)
            .on_input(|s| Message::UpdateInputText(s))
            .padding(10)
            .size(20)
            .icon(Icon {
                font: Font::DEFAULT,
                code_point: '\u{2705}',
                size: None,
                spacing: 10.,
                side: Side::Left,
                }),
            controls,
            text( &self.output_text)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .width(Length::Fill)
            .size(24),
        ].padding(10)
        .align_items(Center)
        .spacing(10);

        container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .into()
    }

    fn theme(&self) -> Theme {
            Theme::Nord

    }
}
