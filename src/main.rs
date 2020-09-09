use iced::Application;
use iced::Element;
use iced::Settings;
use iced::{button,Command,Button, Column, Text, Row};
use iced_native::widget::text_input;

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

//9bytes at most
#[derive(Clone,Copy,Debug)]
enum Token {
    Op(char),
    Num(f64),
    Brace{lhs: bool},
}

fn lexer(inp: String) -> Result<Vec<Token>,AppError>{
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
            if let Some(shift) = state.2 {
                state.0 + state.1 * 10.0f64.powi(-shift)
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
    inp.split(' ').filter(|s| !s.is_empty()).try_fold(Vec::with_capacity(inp.len()/6),|mut acc,it|{
        acc.append(&mut subber(it)?);
        Ok(acc)
    }).map(|mut ok| {Vec::shrink_to_fit(&mut ok);ok})
}

#[derive(Debug,Clone)]
enum TreeData {
    Op(char),
    Num(f64),
}
#[derive(Debug,Clone)]
enum TreeLeaf {
    Unary(Box<TreeNode>),
    Binary{left: Box<TreeNode>, right: Box<TreeNode>}
}
#[derive(Debug,Clone)]
enum TreeNode {
    Ending{data: Option<TreeData>},
    WithChilds{data: Option<char>, children: Option<TreeLeaf>},
}

fn parse(tokens: Vec<Token>) -> Result<TreeNode,AppError> {
    use Token::*;
    let sz = tokens.len();
    let mut acc = 0isize;
    let tokens = tokens.into_iter().try_fold(Vec::with_capacity(sz),|mut vec,it| {
        let mut flag = false;
        match it {
            Brace{lhs: true} => {
                acc+=1;
            },
            Brace{lhs: false} => {
                flag = true;
            },
            _ => {}
        };
        let ret = (acc,it);
        if flag {acc-=1};
        if acc < 0 {return Err(AppError::ParseError("Bad brace formation".to_string()))};
        vec.push(ret);
        Ok(vec)
    })?;
    if acc != 0 {return Err(AppError::ParseError("Bad brace formation".to_string()))};
    println!("{:?}",tokens);

    Err(AppError::ParseError("Not implemented".to_string()))
}

fn eval(tree: TreeNode) -> Result<f64,AppError> {
    match tree {
        TreeNode::Ending{data} => {
            match data {
                Some(TreeData::Num(f)) => {Ok(f)},
                Some(TreeData::Op(_)) => {Err(AppError::EvalError("Found operation in place of number".to_string()))}
                None => {Err(AppError::EvalError("Expected data on leaf".to_string()))}
            }
        }
        TreeNode::WithChilds{data,children} => {
            let childs = match children {
                Some(kids) => kids,
                None => return Err(AppError::EvalError("Children not found".to_string()))
            };
            match data {
                None => {Err(AppError::EvalError("Ill-formed tree".to_string()))},
                Some(op) => {
                    match op {
                        '+' => {
                            match childs {
                                TreeLeaf::Unary(_) => {Err(AppError::EvalError("Found unary op where binary expected".to_string()))},
                                TreeLeaf::Binary{left,right} => {
                                    Ok(eval(*left)? + eval(*right)?)
                                },
                            }
                        },
                        '-' => {
                            match childs {
                                TreeLeaf::Unary(_) => {Err(AppError::EvalError("Found unary op where binary expected".to_string()))},
                                TreeLeaf::Binary{left,right} => {
                                    Ok(eval(*left)? - eval(*right)?)
                                },
                            }
                        },
                        '*' => {
                            match childs {
                                TreeLeaf::Unary(_) => {Err(AppError::EvalError("Found unary op where binary expected".to_string()))},
                                TreeLeaf::Binary{left,right} => {
                                    Ok(eval(*left)? * eval(*right)?)
                                },
                            }
                        },
                        '/' => {
                            match childs {
                                TreeLeaf::Unary(_) => {Err(AppError::EvalError("Found unary op where binary expected".to_string()))},
                                TreeLeaf::Binary{left,right} => {
                                    Ok(eval(*left)? / eval(*right)?)
                                },
                            }
                        },
                        '^' => {
                            match childs {
                                TreeLeaf::Unary(_) => {Err(AppError::EvalError("Found unary op where binary expected".to_string()))},
                                TreeLeaf::Binary{left,right} => {
                                    Ok(eval(*left)?.powf(eval(*right)?))
                                },
                            }
                        },
                        _ => {
                            Err(AppError::EvalError("Bad operation found".to_string()))
                        }
                    }
                },
            }
        }
    }
}



fn perform(inp: String) -> Result<f64,AppError> {
    let tokens = lexer(inp)?;
    let tree = parse(tokens)?;
    eval(tree)
}
fn main() {
    Calculator::run(Settings::default());
}
