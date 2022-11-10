use bool_algebra::{update_values, validate_table, Token};

pub fn latex_table(table: &Vec<bool>, names: &Vec<String>, result_name: &String) -> String {
    if let Err(err) = validate_table(table, names) {
        return format!("{}", err);
    }

    fn get_offset(name: &String) -> (usize, usize) {
        let len = name.len() - 1;
        let right = len / 2;
        let left = len - right;
        (left, right)
    }

    fn print_offset(offset: (usize, usize), value: bool) -> String {
        let mut out = String::new();
        for _ in 0..offset.0 {
            out.push(' ');
        }
        if value {
            out.push('1');
        } else {
            out.push('0');
        }
        out.push('&');

        for _ in 0..offset.1 {
            out.push(' ');
        }
        out
    }

    let mut out = String::new();
    out.push_str("/begin{tabular}{ ");
    for _ in names {
        out.push('c');
        out.push(' ');
    }
    out.push_str("| c }\n ");

    for name in names {
        out.push_str(&format!("{}& ", name));
    }
    out.push_str(&format!(" {}&/hline\n", result_name));

    let offset: Vec<(usize, usize)> = names.iter().map(get_offset).collect();
    let result_offset = get_offset(result_name).1;

    let mut values = vec![false; names.len()];
    let mut index = 0;

    loop {
        out.push(' ');
        for (i, &value) in values.iter().enumerate() {
            out.push_str(&print_offset(offset[i], value));
            out.push(' ');
        }
        out.push_str(&format!(" "));
        for _ in 0..result_offset {
            out.push(' ');
        }

        if table[index] {
            out.push('1');
        } else {
            out.push('0');
        }
        out.push('&');
        out.push('\n');

        if !update_values(&mut values) {
            break;
        }
        index += 1;
    }

    out.push_str("/end{tabular}");

    out
}

pub fn escape(input: String) -> String{
    input
}

pub fn latex_func(func: &Vec<Token>) -> String {
    let mut out = String::new();
    out.push_str("$$");
    for token in func {
        out.push(' ');
        if let Token::Var(name) = token {
            out.push_str(&name);
        } else {
            out.push_str(match token {
                Token::And => "/land",
                Token::Close => "/right)",
                Token::Eq => "",
                Token::ImplicAB => "/leftarrow",
                Token::ImplicBA => "/rightarrow",
                Token::Nand => "/uparrow",
                Token::Nor => "/downarrow",
                Token::Not => "/lnot",
                Token::One => "1",
                Token::Open => "/left(",
                Token::Or => "/lor",
                Token::Var(name) => "",
                Token::Xor => "/oplus",
                Token::Zero => "0",
            });
        }
        out.push(' ');
    }
    out.push_str("$$");

    out
}
