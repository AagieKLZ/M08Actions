#![allow(non_snake_case)]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a number or an arithmetic expression as an argument.");
        return;
    }

    let input = &args[1];
    let val = eval_value(input);
    println!("{}", val);
}

fn eval_value(input: &String) -> String{
    let num: i32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            match eval_expression(input) {
                Ok(result) => {
                    println!("{}", result);
                    return eval_value(&result.to_string());
                },
                Err(_) => {
                    return "Please provide a valid number or arithmetic expression as an argument.".to_string();
                }
            }
        }
    };

    if num < 0 {
        return "buup".to_string();
    } else if num % 2 == 0 {
        return "boop".to_string();
    } else {
        return "beep".to_string();
    }
}

fn eval_expression(input: &str) -> Result<f64, ()> {
    let r = meval::eval_str(input);
    match r {
        Ok(result) => {
            match result.is_finite() {
                true => return Ok(result as f64),
                false => return Err(()),
            }
        },
        Err(_) => {
            return Err(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_even() {
        assert_eq!(eval_value(&"2".to_string()), "boop".to_string());
    }

    #[test]
    fn test_odd() {
        assert_eq!(eval_value(&"3".to_string()), "beep".to_string());
    }

    #[test]
    fn test_negative() {
        assert_eq!(eval_value(&"-1".to_string()), "buup".to_string());
    }

    #[test]
    fn test_expression() {
        assert_eq!(eval_value(&"2+2".to_string()), "boop".to_string());
    }

    #[test]
    fn test_expression2() {
        assert_eq!(eval_value(&"2+2*2".to_string()), "boop".to_string());
    }
}