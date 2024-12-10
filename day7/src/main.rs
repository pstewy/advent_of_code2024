use std::fs;

fn main() {
    let file = fs::read_to_string("src/input_day_7.txt").unwrap();
    let equations = parse(file);
    println!("{}", part1(&equations))
}

#[derive(Debug)]
struct Equation {
    solution: i64,
    values: Vec<i64>,
}

fn parse(input: String) -> Vec<Equation> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut output: Vec<Equation> = Vec::new();
    for line in lines {
        let parts = line.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("bad assumption")
        }
        output.push(Equation {
            solution: parts[0].parse::<i64>().unwrap(),
            values: parts[1]
                .trim_start()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        });
    }
    output
}

fn part1(equations: &Vec<Equation>) -> i64 {
    let mut solvable_solutions = 0;
    for equation in equations {
        if is_solvable(equation) {
            solvable_solutions += equation.solution;
        }
    }
    solvable_solutions
}

#[derive(PartialEq, Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn is_solvable(equation: &Equation) -> bool {
    let mut permutations: Vec<Vec<Operator>> = Vec::new();
    get_operator_permutations(vec![Operator::Add; equation.values.len()-1], 0, &mut permutations);
    for operators in permutations {
       let mut running_value = equation.values[0];
        for (idx, operator) in operators.iter().enumerate() {
           match operator {
               Operator::Add => running_value += equation.values[idx+1],
               Operator::Multiply => running_value *= equation.values[idx+1],
               Operator::Concatenate => running_value = format!("{}{}", running_value, equation.values[idx+1]).parse::<i64>().unwrap(),
           } 
        }
        if running_value == equation.solution {
            return true;
        }
    }
    false
}


fn get_operator_permutations(mut original: Vec<Operator>, current_idx: usize, running: &mut Vec<Vec<Operator>>) {
   if current_idx == original.len() {
       running.push(original.clone());
       return
   }
    original[current_idx] = Operator::Add;
    get_operator_permutations(original.clone(), current_idx + 1, running);
    original[current_idx] = Operator::Multiply;
    get_operator_permutations(original.clone(), current_idx + 1, running);
    original[current_idx] = Operator::Concatenate;
    get_operator_permutations(original.clone(), current_idx + 1, running);
}

#[test]
fn test_get_operator_permutations() {
    let expected_permutations = vec![
        vec![Operator::Add, Operator::Add],
        vec![Operator::Add, Operator::Multiply],
        vec![Operator::Multiply, Operator::Add],
        vec![Operator::Multiply, Operator::Multiply],
    ];   
    let mut actual_permutations: Vec<Vec<Operator>> = Vec::new(); 
   get_operator_permutations(vec![Operator::Add; 2], 0, &mut actual_permutations);
    assert_eq!(expected_permutations, actual_permutations);
}
