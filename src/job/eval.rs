use crate::*;

pub(crate) fn eval(tree: TreeNode) -> Result<f64,AppError> {
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
