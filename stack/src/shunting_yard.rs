use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Operator(char),
    Operand(String),
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn is_operator(c: char) -> bool {
    "+-*/".contains(c)
}

fn infix_to_postfix(expression: &str) -> String {
    let mut output: VecDeque<Token> = VecDeque::new();
    let mut operators: Vec<char> = Vec::new();

    let mut num_buf = String::new();

    for c in expression.chars() {
        if c.is_whitespace() {
            continue; // 忽略空白符
        }
        if c.is_digit(10) {
            num_buf.push(c); // 收集数字
        } else {
            if !num_buf.is_empty() {
                output.push_back(Token::Operand(num_buf.clone())); // 将数字推入输出队列
                num_buf.clear();
            }

            if c == '(' {
                operators.push(c);
            } else if c == ')' {
                while let Some(op) = operators.pop() {
                    if op == '(' {
                        break;
                    }
                    output.push_back(Token::Operator(op));
                }
            } else if is_operator(c) {
                while let Some(&op) = operators.last() {
                    if precedence(op) >= precedence(c) {
                        output.push_back(Token::Operator(operators.pop().unwrap()));
                    } else {
                        break;
                    }
                }
                operators.push(c);
            }
        }
    }

    // 将缓冲区中的剩余数字推入输出队列
    if !num_buf.is_empty() {
        output.push_back(Token::Operand(num_buf.clone()));
    }

    // 将剩余的操作符推入输出队列
    while let Some(op) = operators.pop() {
        output.push_back(Token::Operator(op));
    }

    let mut result = String::new();
    for token in output {
        match token {
            Token::Operand(n) => result.push_str(&n),
            Token::Operator(op) => result.push(op),
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_test() {
        let expression = "3 + 5 * ( 2 - 8 )";
        let postfix = infix_to_postfix(expression);

        println!("result is: {:?}", postfix);
    }
}
