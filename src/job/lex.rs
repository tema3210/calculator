// use std::pin::Pin;
use crate::*;

pub(crate) fn lexer(inp: String) -> Result<Vec<Token>,AppError>{
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
    inp.split(' ').filter(|s| !s.is_empty()).try_fold(Vec::with_capacity(inp.len()/2),|mut acc,it|{
        acc.append(&mut subber(it)?);
        Ok(acc)
    }).map(|mut ok| {Vec::shrink_to_fit(&mut ok);ok})
}

fn lexer2<'a>(input: &'a str) -> impl Iterator<Item = Token> {

    let gen = |token: Option<char>| {

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
                match token {
                    Some(ch) if op_pred(ch) => {
                        if num_state.3 {
                            yield (Some(Token::Num(dump_numb(num_state))),false);
                            num_state = (0.0,0.0,None,false);
                        }
                        yield (Some(Token::Op(ch)),true);
                    },
                    Some(ch) if brace_pred(ch) => {
                        if num_state.3 {
                            yield (Some(Token::Num(dump_numb(num_state))),false);
                            num_state = (0.0,0.0,None,false);
                        }
                        yield (Some(Token::Brace{lhs: ch == '('}),true);
                        //ret.push(Token::Brace{lhs: ch == '('});
                    },
                    Some(ch) if ch.is_digit(10) => {
                        num_state.3 = true;
                        if let Some(ref mut dp) = num_state.2 {
                            num_state.1 = num_state.1 * 10. + ch.to_digit(10).unwrap() as f64;
                            *dp+=1;
                        } else {
                            num_state.0 = num_state.0 * 10. + ch.to_digit(10).unwrap() as f64;
                        };
                        yield (None,true);
                    },
                    Some('.') => {
                        if num_state.3 == false {break Err(AppError::LexError("Found dot outside of number".to_string()))};
                        if num_state.2.is_some() {
                            break Err(AppError::LexError("Found number with 2 dots".to_string()))
                        } else {
                            num_state.2 = Some(0)
                        };
                        yield (None,true);
                    },
                    Some(' ') => {
                        if num_state.3 {
                            yield (Some(Token::Num(dump_numb(num_state))),true);
                            num_state = (0.0,0.0,None,false);
                        } else {
                            yield (None,true);
                        };

                    },
                    Some(_) => {
                        break Err(AppError::LexError("Found improcessable char".to_string()))
                    },
                    None => {
                        if num_state.3 {
                            //ret.push(Token::Num(dump_numb(num_state)));
                            yield (Some(Token::Num(dump_numb(num_state))),false);
                        }
                        break Ok(());
                    }
                }
            }?;

            Ok(())
    };
    let mut gen = Box::pin(gen);

}
