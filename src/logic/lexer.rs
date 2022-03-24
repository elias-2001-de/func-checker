use bool_algebra::Token;

pub fn lex(func: &str, operators: &Vec<char>) -> Result<Vec<Token>, String> {
    for operator in operators {
        if is_alphanumeric(operator.clone()) {
            return Err("the operators can't be an alphanumeric".to_string());
        }
    }

    let token_map = vec![
        Token::And,
        Token::Or,
        Token::Xor,
        Token::Not,
        Token::Nor,
        Token::Nand,
        Token::ImplicAB,
        Token::ImplicBA,
        Token::Eq,
    ];

    if operators.len() > token_map.len() {
        return Err(format!(
            "to much operators definde max operator {}",
            token_map.len()
        ));
    }

    let mut result = Vec::new();
    let mut skip = 0;
    let mut name = String::new();

    for (i, c) in func.chars().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }
        match c {
            '1' => result.push(Token::One),
            '0' => result.push(Token::Zero),

            '(' => result.push(Token::Open),
            ')' => result.push(Token::Close),

            ' ' => (),
            '\t' => (),

            _ => {
                let mut flag = true;
                for i in 0..operators.len() {
                    if c == operators[i] {
                        result.push(token_map[i].clone());
                        flag = false;
                        break;
                    }
                }
                if flag {
                    match lex_var(func, i, &mut name) {
                        Err(err) => return Err(err),
                        Ok(s) => {
                            result.push(Token::Var(name));
                            name = String::new();
                            skip = s;
                        }
                    }
                }
            }
        }
    }

    if let Err(err) = bool_algebra::validate_func(&result) {
        return Err(err);
    }

    Ok(result)
}

fn lex_var(func: &str, i: usize, name: &mut String) -> Result<u32, String> {
    if !is_letter(func.chars().nth(i).unwrap()) {
        return Err(format!(
            "unexpected char {} at index {}",
            func.chars().nth(i).unwrap(),
            i
        ));
    }

    name.push(func.chars().nth(i).unwrap());
    let mut skip = 0;
    let mut i = i + 1;

    while let Some(c) = func.chars().nth(i) {
        if !is_alphanumeric(c) {
            break;
        }
        name.push(c);
        skip += 1;
        i += 1;
    }

    Ok(skip)
}

fn is_letter(c: char) -> bool {
    for l in "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".chars() {
        if l == c {
            return true;
        }
    }
    return false;
}

fn is_digit(c: char) -> bool {
    for l in "0123456789".chars() {
        if l == c {
            return true;
        }
    }
    return false;
}

fn is_alphanumeric(c: char) -> bool {
    return is_letter(c) || is_digit(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_letter() {
        assert_eq!(is_letter('a'), true);
        assert_eq!(is_letter('A'), true);
        assert_eq!(is_letter('1'), false);
        assert_eq!(is_letter('ä'), false);
        assert_eq!(is_letter('t'), true);
    }

    #[test]
    fn test_is_digit() {
        assert_eq!(is_digit('a'), false);
        assert_eq!(is_digit('A'), false);
        assert_eq!(is_digit('1'), true);
        assert_eq!(is_digit('ä'), false);
        assert_eq!(is_digit('8'), true);
    }

    #[test]
    fn test_lex_var() {
        let mut name = String::new();
        assert_eq!(lex_var(" 1 | abc", 5, &mut name), Ok(2));
        assert_eq!(name, "abc".to_string());
    }

    #[test]
    fn test_lex() {
        assert_eq!(
            lex("b & c | c", &vec!['&', '|', '^', '!', '>', '<', '=']),
            Ok(vec![
                Token::Var("b".to_string()),
                Token::And,
                Token::Var("c".to_string()),
                Token::Or,
                Token::Var("c".to_string()),
            ])
        );
    }
}
