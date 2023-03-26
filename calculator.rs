use std::io;
use crate::math::Operators;

fn main() {
    println!("Welcome to calculator!\nplease specify the operator (+, -, *, /, exit).");
    loop {
        let op = get_inp(Some("operator:\t"));
        match &op.as_str() {
            &"*" | &"+" | &"-" | &"/"| &"**" => {
                let first = get_inp(Some("First number:\t"));
                let second = get_inp(Some("Second number:\t"));
                let opt = get_operator(op.as_str());
                match opt {
                    Ok(o) => {
                        let operation_parts = math::OperationParts { operation: o, first_num: first.clone(), second_num: second.clone() };
                        let res = math::eval(operation_parts);
                        match res {
                            Ok(r) => println!("{} {} {} = {}\n", first, op, second, r),
                            Err(e) => println!("Err! {}", e)
                        }
                    },
                    Err(e) => {
                        println!("Error!: {}", e)
                    }
                }
            },
            &"exit" => { break; },
            &_ => { println!("Not understood! try again."); continue; }
        }
    }
    println!("cya later :)");
}

fn get_operator(op: &str) -> Result<math::Operators, math::OperationError> {
    match op {
        "/" => Ok(Operators::Division),
        "*" => Ok(Operators::Multiply),
        "+" => Ok(Operators::Add),
        "-" => Ok(Operators::Minus),
        _ => Err(math::OperationError::OperatorNotDefinedOrNotSpecified)
    }
}

fn get_inp(msg: Option<&str>) -> String {
    match msg {
        Some(x) => println!("{}", x),
        None => {}
    }
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Err while proccessing input");
    input.trim_end().to_string()
}

mod math {
    ///////// FNS ////////////
    fn parsable (s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }
    pub fn eval (opt: OperationParts<String>) -> Result<f64, OperationError> {
        if parsable(&opt.first_num) && parsable(&opt.second_num){
            let n1: f64 = opt.first_num.parse().unwrap();
            let n2: f64 = opt.second_num.parse().unwrap();
            match opt.operation {
                Operators::Division => { 
                    if n2 != 0.0 {
                        Ok(n1 / n2)
                    } else {
                        Err(OperationError::ZeroDivisionError)
                    }
                 },
                Operators::Multiply => { Ok(n1 * n2) }
                Operators::Add => { Ok(n1 + n2) },
                Operators::Minus => { Ok(n1 - n2) },
            }
        } else {
            Err(OperationError::NotANumberError)
        }
    }
    ////////// ENUMS //////////
    pub enum Operators {
        Division,
        Multiply,
        Add,
        Minus
    }
    ////////// STRUCTS /////////
    pub struct OperationParts <T> {
        pub operation: Operators,
        pub first_num: T,
        pub second_num: T
    }

    //////// ERROR HANDLINGS //////////
    #[derive(PartialEq, Debug)]
    pub enum OperationError {
        ZeroDivisionError,
        NotANumberError,
        OperatorNotDefinedOrNotSpecified
    }
    impl std::fmt::Display for OperationError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let description = match *self {
                OperationError::OperatorNotDefinedOrNotSpecified => "Cannot comprehend entered operator.",
                OperationError::NotANumberError => "Not a number.",
                OperationError::ZeroDivisionError => "Trying to divide something by zero."
            };
            f.write_str(description)
        }
    }
    impl std::error::Error for OperationError {}
}
