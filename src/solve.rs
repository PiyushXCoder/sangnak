use std::fmt;
use std::error::Error;

use Khand::*;
use Opr::*;

#[derive(Debug)]
enum Khand {
    NUM(f64),
    OPR(Opr),
    NIL
}

#[derive(Debug)]
enum Opr {
    DIV,
    MUL,
    ADD,
    SUB
}

#[derive(Debug)]
pub struct WorngStatementError;

impl fmt::Display for WorngStatementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for WorngStatementError {}

pub fn solve(exp: &str) -> Result<f64, WorngStatementError> {
	let chars = exp.replace(" ", "").chars().collect::<Vec<char>>();
	
	match solve_expression(&chars, None) {
		Ok((out, _)) => Ok(out),
		Err(err) => Err(err)
	}
}

fn solve_expression(exp: &Vec<char>, start: Option<usize>) -> Result<(f64, usize), WorngStatementError> {
    // println!("start: {:?}", start);
    let mut khand: Vec<Khand> = Vec::new();

    let mut div: Vec<usize> = Vec::new();
    let mut mul: Vec<usize> = Vec::new();
    let mut add: Vec<usize> = Vec::new();
    let mut sub: Vec<usize> = Vec::new();
    
    let mut buf = String::new();
    let mut index: usize = start.unwrap_or(0); 
    while index < exp.len() {
        let chr = exp[index];
        // println!("LOOP: ({}) {:?}", chr, khand);

        let is_part_of_number = ('0' <= chr && chr <= '9') || (buf.len() > 0 && chr == '.');
        if is_part_of_number {
            buf.push(chr);
            // println!(" BUF: {}", buf);
        }

        if (!is_part_of_number || (index+1 >= exp.len())) && buf.len() > 0 {
            match buf.parse::<f64>() {
                Ok(num) => khand.push(NUM(num)),
                Err(_) => return Err(WorngStatementError)
            }
            buf = String::new();
        }

        if chr == '(' {
            let (out,i) =  solve_expression(exp, Some(index +1))?;
            khand.push(NUM(out));
            index = i;
        } else if chr == ')' && start.is_some() {
            break;
        }

        match chr {
            '/' => {
                div.push(khand.len());
                khand.push(OPR(DIV));
            },
            '*' => {
                mul.push(khand.len());
                khand.push(OPR(MUL));
            },
            '+' => {
                add.push(khand.len());
                khand.push(OPR(ADD));
            },
            '-' => {
                sub.push(khand.len());
                khand.push(OPR(SUB));
            },
            _ => {}
        }

        index += 1;
    }

    // println!("AFTER LOOP: {:?}", khand);

    for i in div {
        let (left, left_index) = get_left(&khand, i)?;
        let (right, right_index) = get_right(&khand, i)?;
        khand[i] = NUM(left/right);
        khand[left_index] = NIL; 
        khand[right_index] = NIL; 
        // println!("DIV: {:?}", khand);
    }
    
    for i in mul {
        let (left, left_index) = get_left(&khand, i)?;
        let (right, right_index) = get_right(&khand, i)?;
        khand[i] = NUM(left*right);
        khand[left_index] = NIL; 
        khand[right_index] = NIL;
        // println!("MUL: {:?}", khand);
    }

    for i in add {
        let (left, left_index) = get_left(&khand, i)?;
        let (right, right_index) = get_right(&khand, i)?;
        khand[i] = NUM(left+right);
        khand[left_index] = NIL; 
        khand[right_index] = NIL;
        // println!("ADD: {:?}", khand);
    }

    for i in sub {
        let (left, left_index) = get_left(&khand, i)?;
        let (right, right_index) = get_right(&khand, i)?;
        khand[i] = NUM(left-right);
        khand[left_index] = NIL; 
        khand[right_index] = NIL;
        // println!("SUB: {:?}", khand);
    }

    // println!("CLOSING: {}", get_right(&khand, 0)?.0);
    Ok((get_right(&khand, 0)?.0, index))
}

fn get_left(khand: &Vec<Khand>, i: usize) -> Result<(f64, usize), WorngStatementError> {
    if khand.len() == 1 {
        match khand[0] {
            NUM(num) => return Ok((num, i)), 
            _ => return Err(WorngStatementError)
        }
    }

    let mut i = i;
    loop {
        i -= 1;
        if let NUM(num) = match khand.get(i) {
            Some(num) => num,
            None => return Err(WorngStatementError)
        } {
            return Ok((*num, i));
        }
    }
}


fn get_right(khand: &Vec<Khand>, i: usize) -> Result<(f64, usize), WorngStatementError> {
    if khand.len() == 1 {
        match khand[0] {
            NUM(num) => return Ok((num, i)), 
            _ => return Err(WorngStatementError)
        }
    }  

    let mut i = i;
    loop {
        i += 1;
        if let NUM(num) = match khand.get(i) {
            Some(num) => num,
            None => return Err(WorngStatementError)
        } {
            return Ok((*num, i));
        }
    }    
}

