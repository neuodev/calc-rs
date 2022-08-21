use regex::Regex;

enum Op {
    Plus,
    Mins,
    Multi,
    Div,
}

fn split<'a>(expr: &'a str, by: &'a str) -> Vec<&'a str> {
    expr.split(by).map(|term| term.trim()).collect()
}

fn calc(expr: &str) -> f64 {
    let re = Regex::new(r"(\(|\[)(?P<expr>[^\)\]]+)(\)|\])").unwrap();
    let mut temp_expr = expr.to_string();
    // println!("temp expr: {}", temp_expr);
    re.captures_iter(expr).for_each(|cap| {
        let res = calc(&cap["expr"]);

        println!("Cap: {:#?}", cap);
        println!("result: {:#?}", res);
        println!("curr expr: {}", temp_expr);
        println!("next expr: {}", temp_expr.to_string().replace(&cap[0],  &res.to_string()));
        temp_expr = temp_expr.to_string().replace(&cap[0],  &res.to_string());
    });


    let expr = temp_expr.as_str();
    // println!("Expr: {}", expr);
    let mut terms = split(expr, "+");
    let mut op = Op::Plus;

    if terms.len() == 1 {
        terms = split(expr, "-");
        op = Op::Mins;
    }

    if terms.len() == 1 {
        terms = split(expr, "x");
        op = Op::Multi;
    }

    if terms.len() == 1 {
        terms = split(expr, "*");
        op = Op::Multi;
    }

    if terms.len() == 1 {
        terms = split(expr, "/");
        op = Op::Div;
    }


    let mut result: f64 = 0.0;
    terms.into_iter().enumerate().for_each(|(idx, term)| {    
        let term = match term.parse() {
            Ok(num) => num,
            Err(_) => calc(term)
        };

        if idx == 0 {
            result = term;
            return 
        } 

        match op {
            Op::Plus => result += term,
            Op::Mins => result -= term,
            Op::Multi => result *= term,
            Op::Div => result /= term,
        }
    
    });

    result
}
fn main() {
    let expr = "[1 * 2] * [1 + 1]";
    let result = calc(expr);
    println!("{} = {}", expr, result)
}
