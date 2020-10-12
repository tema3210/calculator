use crate::*;

pub(crate) fn eval(tree: TreeNode) -> Result<f64,AppError> {
    match tree {
        TreeNode::Ending(Some(num)) => Ok(num),
        TreeNode::Ending(None) => Err(AppError::EvalError("Ill formed ending".into())),
        TreeNode::WithChilds{data: Some(op),children: Some(ch)} => {
            let (right,left) = *ch;
            
            let f = move |fst: f64,sec: f64| -> Result<f64,AppError> {
                match op {
                    '*' => Ok(fst * sec),
                    '/' => Ok(fst / sec),
                    '+' => Ok(fst + sec),
                    '-' => Ok(fst - sec),
                    '^' => Ok(fst.powf(sec)),
                    _ => Err(AppError::EvalError("Bad op found".into()))
                }
            };

            f(eval(left)?,eval(right)?)
        },
        TreeNode::WithChilds{data: _, children: _} => {
            Err(AppError::EvalError("Ill formed tree".into()))
        },
    }
}
