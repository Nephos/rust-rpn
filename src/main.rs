use std::io;

fn main() {
    println!("Input your calculation.");
    println!("Press Enter to validated.");

    // let token_num = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let input_line = input.trim();
    let input_split = input_line.split(' ');
    let input_tab: Vec<&str> = input_split.collect();
    //let input_tab = input_split.collect::<Vec<&str>>();

    let token_op1 = ["+", "-"];
    let token_op2 = ["*", "/", "%"];
    let token_opall = ["+", "-", "*", "/", "%"];
    let token_sep_open = "(";
    let token_sep_close = ")";

    let mut stack_out: Vec<String> = vec![];
    let mut stack_ope: Vec<String> = vec![];

    for token_ref in input_tab.iter() {
        let token = token_ref.to_string();
        let token_ref: &str = &token;
        match token.parse::<f64>() {
            Ok(..) => {
                println!("find f64 {}", token);
                stack_out.push(token.to_string());
            },
            Err(..) => {
                if token_opall.contains(&token_ref) {
                    println!("find operator {}", token);
                    if stack_ope.len() > 0 {
                        let token_last = stack_ope[stack_ope.len()-1].to_string();
                        let token_last_ref: &str = &token_last;
                        if token_op1.contains(&token_ref)
                            || (token_op2.contains(&token_ref) && token_op2.contains(&token_last_ref)) {
                                stack_out.push(token_last.to_string());
                                stack_ope.pop();
                                println!("push {} from the stack to the out", token_last);
                            }
                    }
                    println!("push {} to the stack", token);
                    stack_ope.push(token.to_string());
                }
                else if token == token_sep_open {
                    stack_ope.push(token.to_string());
                }
                else if token == token_sep_close && stack_ope.len() > 0 {
                    let mut token_last = stack_ope[stack_ope.len()-1].to_string();
                    while token_last != token_sep_open {
                        stack_out.push(token_last.to_string());
                        stack_ope.pop();
                        token_last = stack_ope[stack_ope.len()-1].to_string();
                    }
                    stack_ope.pop();
                }
                else {
                    println!("Error.");
                }
            },
        }
    }

    while stack_ope.len() > 0 {
        let token_last = stack_ope[stack_ope.len()-1].to_string();
        println!("empty the stack, push {} to out", token_last);
        stack_out.push(token_last.to_string());
        stack_ope.pop();
    }

    for out in stack_out.iter() {
        println!("Calculation {}", out);
        let string_to_find: &str = out;
        if token_opall.contains(&string_to_find) {
            let n2: f64;
            let n1: f64;
            n2 = stack_ope[stack_ope.len()-1].trim().parse::<f64>().ok().unwrap();
            stack_ope.pop();
            n1 = stack_ope[stack_ope.len()-1].trim().parse::<f64>().ok().unwrap();
            stack_ope.pop();
            let mut total: f64;
            if out == "+" {
                total = n1 + n2;
            }
            else if out == "-" {
                total = n1 - n2;
            }
            else if out == "*" {
                total = n1 * n2;
            }
            else if out == "/" {
                total = n1 / n2;
            }
            else if out == "%" {
                total = n1 % n2;
            }
            else {
                total = 0.0;
            }
            let total_string = total.to_string();
            stack_ope.push(total_string);
        }
        else {
            stack_ope.push(out.to_string());
        }
    }

    if stack_ope.len() > 0 {
        let result = stack_ope[stack_ope.len() - 1].to_string();
        println!("{} = {}", input_line, result);
    }
    else {
        println!("No operation todo");
    }
}

// Si nombre
//  -> OUT
// Si Operateur
//  o1: nouveau
//  o2: tete de pile
//  Si o1 <= o2
//   -> o2 sort de la PILE
//   -> o2 va dans OUT
//  -> o1 sur la PILE
// Si (
//  -> PILE
// Si )
//  -> DEPILE sur OUT jusqu'à )
