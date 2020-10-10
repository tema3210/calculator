use iced::Element;
use iced::{button,Command,Button, Column, Text, Row};
use iced_native::widget::text_input;

#[derive(Default)]
pub(crate) struct Calculator {
    btn: button::State,
    txt_st: text_input::State,

    out: String,

    status_str: String,
}

impl iced::Application for Calculator {
    type Executor = iced::executor::Default;
    type Message = calc::Msg;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>){
        (Self::default(),Command::none())
    }

    fn title(&self) -> String {
        String::from("Calculator!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use calc::Msg::*;
        match message {
            EnterPressed => {
                let cl = self.out.clone();
                Command::perform(async move { calc::perform(cl) },|res| {
                    match res {
                        Ok(num) => {
                            Self::Message::Output(num.to_string())
                        },
                        Err(e) => {
                            Self::Message::Status(String::from(format!("{:?}",e)))
                        },
                    }
                })
            },
            TextChange(s) => {
                self.out = s;
                Command::none()
            },
            Output(s) => {
                self.out = s;
                Command::none()
            },
            Status(s) => {
                self.status_str = s;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message>{
        Column::new()
            .padding(20)
            .push(
                Text::new("Characters which aren't used in equation notation will be filtered out")
                .size(15) //px
            )
            .push(
                Text::new("Lexems are separated by the space, so don't expect things like \"9 + - 10 - - 0\" to be processed correctly ")
                .size(15) //px
            )
            .push(
                Row::new()
                    .push(
                        text_input::TextInput::new(&mut self.txt_st,"",&self.out,
                            |changed| {
                                Self::Message::TextChange(changed)
                            }
                        ),
                    )
                    .push(
                        Button::new(&mut self.btn,Text::new("Enter"))
                            .on_press(Self::Message::EnterPressed),
                    )
            )
            .push(
                Row::new()
                    .push(
                        Text::new(&self.status_str),
                    )
            )
            .into()
    }
}
