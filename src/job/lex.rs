use crate::*;

#[inline]
pub(crate) fn lexer(inp: &str) -> Result<Vec<Token>,AppError>{
    let subber = |item: &str| -> Result<Vec<Token>,AppError> {
        let mut ret = Vec::new();
        let mut it = item.chars().peekable();

        let op_pred = |ch: char| -> bool {
            ['+','-','*','/','^'].iter().position(|&c| c == ch).is_some()
        };
        let brace_pred = |ch: char| -> bool {
            if ch == '(' || ch == ')' { true } else { false }
        };

        //number before dot, number after dot, position of dot?, presence of number?,is negative?
        let mut num_state: (f64,f64,Option<i32>,bool,bool) = (0.0,0.0,None,false,false);

        // let mut op_cache: Option<char> = None;

        let dump_numb = |state: (f64,f64,Option<i32>,bool,bool)| -> f64 {
            let sgn: f64 = if state.4 { -1.0 } else { 1.0 };
            let ret = if let Some(shift) = state.2 {
                state.0 + state.1 * 10.0f64.powi(-shift)
            } else {
                state.0
            };
            ret * sgn
        };
        loop {
            match it.next() {
                Some(ch) if op_pred(ch) => {
                    //is in number? is number negative?
                    match (num_state.3,num_state.4) {
                        (true,_) => {
                            ret.push(Token::Num(dump_numb(num_state)));
                            num_state = (0.0,0.0,None,false,false);
                            ret.push(Token::Op(ch));
                        },
                        (false,true) => {
                            unreachable!("negative flag outside of number")
                        },
                        (false,false) => {
                            match it.peek() {
                                Some(ch) if ch.is_digit(10) => {
                                    num_state.4 = true;
                                    num_state.3 = true;
                                },
                                _ => {
                                    ret.push(Token::Op(ch));
                                },
                            };
                        }
                    }
                },
                Some(ch) if brace_pred(ch) => {
                    if num_state.3 {
                        ret.push(Token::Num(dump_numb(num_state)));
                        num_state = (0.0,0.0,None,false,false);
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
                    if num_state.3 == false {break Err(AppError::LexError("Found dot outside of number".into()))};
                    if num_state.2.is_some() {
                        break Err(AppError::LexError("Found number with 2 dots".into()))
                    } else {
                        num_state.2 = Some(0)
                    }
                },
                Some(_) => {
                    break Err(AppError::LexError("Found improcessable char".into()))
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

#[cfg(test)]
mod lexer_tests {
    use crate::job::lex::lexer;

    #[test]
    fn t_01(){
        println!("{:?}", lexer("( -90 * 7 ) + 5 / 1 - 8^(12 - 2)".into()) );
    }

    #[test]
    fn t_02(){
        println!("{:?}", lexer("( -90 * 7 ) + 5 / 1 - 8^(12 - -2)".into()));
    }
    #[test]
    fn t_03() {
        println!("{:?}", lexer("( -90 * -7 ) + 5 / -1 - 8^-2".into()) );
    }
}
