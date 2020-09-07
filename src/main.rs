use iced::Application;
use iced::Element;
use iced::Settings;
use iced::{button,Command,Button, Column, Text, Row};
use iced_native::widget::text_input;
use regex::*;

#[macro_use]
extern crate lazy_static;

#[derive(Default)]
struct Calculator {
    btn: button::State,
    txt_st: text_input::State,

    out: String,

    status_str: String,
}

#[derive(Debug,Clone)]
enum Msg {
    EnterPressed,
    TextChange(String),
    Output(String),
    Status(String),
}

enum AppError {
    ParseError(String),
    Test(String),
}

impl iced::Application for Calculator {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>){
        (Self::default(),Command::none())
    }

    fn title(&self) -> String {
        String::from("Calculator!")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Msg::*;
        match message {
            EnterPressed => {
                let cl = self.out.clone();
                Command::perform(async move { perform(cl) },|res| {
                    match res {
                        Ok(num) => {
                            Self::Message::Output(num.to_string())
                        },
                        Err(AppError::ParseError(s)) => {
                            Self::Message::Status(String::from("Bad input: Error in evaluating; ") + &s)
                        },
                        Err(AppError::Test(s)) => {
                            Self::Message::Status("TEST: ".to_string() + &s)
                        }
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

#[derive(Clone,Copy)]
enum Token {
    Num(f64),
    Op(char),
    Brace{lhs: bool},
}

impl Token {
    fn to_string(self) -> String {
        match self {
            Token::Num(f) => format!("Num({})",f).into(),
            Token::Op(c) => format!("Op({})",c).into(),
            Token::Brace{lhs} => format!("Brace({})",if lhs {'('} else {')'}).into(),
        }
    }
}

fn perform(input: String) -> Result<f64,AppError> {
    const OP_TOKENS: [char;7] = ['(',')','+','-','*','/','^'];
    let mut err: Option<Vec<&'static str>> = None;
    let tokens: Vec<_> = input.split(r" ").filter(|s| !s.is_empty()).map(|semi_token: &str| -> Result<Vec<Token>,&'static str> {
        use Token::*;
        let mut it = semi_token.chars()
            .filter(
                |&ch| ch == '.' ||
                    char::is_numeric(ch) ||
                    OP_TOKENS.iter().fold(false,|acc,&item| acc || (item == ch))
            );
        //here goes number before dot, number after dot, dot position, presence of dot
        let (mut num_bd,mut num_ad,mut ad_shift, mut dot_found) = (0.0f64,0.0f64,0i32,false);
        let mut isNum = false;
        loop {
            let mut vc = Vec::new();
            let mut num_producer = |reset: bool| -> f64 {
                if !dot_found {
                    if reset {num_bd=0.;num_ad=0.;ad_shift=0;dot_found=false;};
                    num_bd
                } else {
                    if reset {num_bd=0.;num_ad=0.;ad_shift=0;dot_found=false;};
                    num_bd + 10.0f64.powi(-ad_shift) * num_ad
                }
            };


            match it.next() {
                Some(ch) if OP_TOKENS.iter().position(|&i| i == ch).is_some() || ch.is_digit(10) || ch == '.' => {
                    match ch {
                        '(' => {
                            if isNum {
                                vc.push(Num(num_producer(true)));
                                    isNum = false;
                                };
                                vc.push(Brace{lhs: true});
                            },
                        ')' => {
                            if isNum {
                                vc.push(Num(num_producer(true)));
                                isNum = false;
                            }
                            vc.push(Brace{lhs: false});
                        },
                        '+' | '-' | '*' | '/' | '^' => {
                            if isNum {
                                vc.push(Num(num_producer(true)));
                                isNum = false;
                            }
                            vc.push(Op(ch));
                        },
                        x if x.is_digit(10) => {
                            if !isNum { isNum = true;num_producer(true);} //setting to number parsing mode
                            else {
                                if dot_found {
                                    num_ad = num_ad * 10.0 + x.to_digit(10).unwrap() as f64;
                                    ad_shift+=1;
                                } else {
                                    num_bd = num_bd * 10.0 + x.to_digit(10).unwrap() as f64;
                                }
                            }
                        },
                        '.' => {
                            if isNum {
                                if !dot_found { dot_found = true; }
                                else {break Err("Second dot in number found")}
                            } else {
                                break Err("Found dot ouside of number")
                            }
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                },
                Some(_) => {break Err("Found impossible char")},
                None => {
                    if isNum { vc.push(Num(num_producer(true)))}
                    break Ok(vc)
                }
            }
        }
    }).map(|s| Result::unwrap_or_else(s,|e| {
        match err {
            Some(ref mut vec) => {
                vec.push(e);
            },
            None => {
                err = Some(Vec::new())
            }
        };
        vec![]
    })).flatten().collect();

    if let Some(err_vec) = err {
        return Err(AppError::ParseError(err_vec.into_iter().fold(String::new(),|acc,s| {acc + "; " + s})));
    };
    //tokens is vector of tokens
    return Err(AppError::Test(input +" given " + &tokens.iter().fold(String::new(),|acc,&it| acc + " " + &(it.to_string()))));


    // Err(AppError::ParseError("Unimplemented".to_string()))
}



fn main() {
    Calculator::run(Settings::default())
}
