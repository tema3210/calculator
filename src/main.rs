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

#[derive(Debug)]
enum AppError {
    ParseError(String),
    EvalError(String),
    LexError(String),
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

#[derive(Clone,Copy,Debug)]
enum Token {
    Op(char),
    Num(f64),
    Brace{lhs: bool},
}
impl Token {
    fn to_string(self) -> String {
        use crate::Token::*;
        match self {
            Op(ch) => format!("Op({})",ch),
            Num(f) => format!("Num({})",f),
            Brace{lhs} => format!("Brace('{}')",if lhs {'('} else {')'}),
        }.into()
    }
}

fn lexer(inp: String) -> Result<Vec<Token>,AppError>{
    let strs = inp.split(' ').filter(|s| !s.is_empty());

    let subber = |item: &str| -> Result<Vec<Token>,AppError> {
        let mut ret = Vec::new();
        let mut it = item.chars();

        let op_pred = |ch: char| -> bool {
            ['+','-','*','/','^'].iter().position(|&c| c == ch).is_some()
        };
        let brace_pred = |ch: char| -> bool {
            if ch == '(' || ch == ')' { true } else { false }
        };

        //number before dot, number after dot, position of dot?, presence of number?
        let mut num_state: (f64,f64,Option<i32>,bool) = (0.0,0.0,None,false);
        let dump_numb = |state: (f64,f64,Option<i32>,bool)| -> f64 {
            if state.2.is_some() {
                state.0 + state.1 * 10.0f64.powi(-state.2.unwrap())
            } else {
                state.0
            }
        };
        loop {
            match it.next() {
                Some(ch) if op_pred(ch) => {
                    if num_state.3 {
                        ret.push(Token::Num(dump_numb(num_state)));
                        num_state = (0.0,0.0,None,false);
                    }
                    ret.push(Token::Op(ch));
                },
                Some(ch) if brace_pred(ch) => {
                    if num_state.3 {
                        ret.push(Token::Num(dump_numb(num_state)));
                        num_state = (0.0,0.0,None,false);
                    }
                    ret.push(Token::Brace{lhs: ch == '('});
                },
                Some(ch) if ch.is_digit(10) => {
                    num_state.3 = true;
                    if let Some(ref mut dp) = num_state.2 {
                        num_state.1 = num_state.1 * 10. + ch.to_digit(10).unwrap() as f64;
                        *dp+=1;
                    } else {
                        num_state.0 = num_state.0 * 10. + ch.to_digit(10).unwrap() as f64;
                    }
                },
                Some('.') => {
                    if num_state.3 == false {break Err(AppError::LexError("Found dot outside of number".to_string()))};
                    if num_state.2.is_some() {
                        break Err(AppError::LexError("Found number with 2 dots".to_string()))
                    } else {
                        num_state.2 = Some(0)
                    }
                },
                Some(_) => {
                    break Err(AppError::LexError("Found improcessable char".to_string()))
                },
                None => {
                    if num_state.3 {
                        ret.push(Token::Num(dump_numb(num_state)))
                    }
                    break Ok(());
                }
            }
        }?;

        Ok(ret)
    };

    let mut ret = Vec::new();
    for i in strs {
        ret.append(&mut subber(i)?);
    };
    Ok(ret)
}
fn parse(tokens: Vec<Token>) -> Result<(),AppError> {
    //println!("{:?}",tokens.into_iter().map(Token::to_string).collect::<Vec<_>>());
    Ok(())
}
fn eval(tree: ()) -> Result<f64,AppError> {
    Ok(0.0)
}

fn perform(inp: String) -> Result<f64,AppError> {
    eval(parse(lexer(inp)?)?)
}



fn main() {
    Calculator::run(Settings::default());
}
