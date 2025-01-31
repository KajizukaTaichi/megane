use rustyline::DefaultEditor;

fn main() {
    println!("Megane: array-oriented rewrite computing model");
    println!("(c) 2024 梶塚太智 All rights reversed.\n");
    let mut rl = DefaultEditor::new().unwrap();

    println!("Input meganes that defines rule");
    let mut meganes = vec![];
    loop {
        let inputed = rl.readline("> ").unwrap().trim().to_string();
        if inputed.is_empty() {
            break;
        }
        for inputed in inputed.split(";") {
            let inputed: Vec<&str> = inputed.split("=").collect();
            meganes.push(Megane {
                before: inputed[0]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
                after: inputed[1].trim().to_string(),
            });
        }
    }

    println!("Meganes: {:?}", &meganes);
    loop {
        println!("Input expression to watching");
        let inputed = rl.readline("> ").unwrap().trim().to_string();
        if inputed.is_empty() {
            break;
        }

        let expr = inputed.split_whitespace().map(|s| s.to_string()).collect();
        println!("Expr: {:?}", &expr);
        let result = watch(expr, &meganes);
        if let Some(result) = result {
            println!("Result: {:?}", result);
        } else {
            println!("No result");
        }
    }
}

#[derive(Clone, Debug)]
struct Megane {
    before: Vec<String>,
    after: String,
}

fn search(meganes: &Vec<Megane>, target: Vec<String>) -> Option<Megane> {
    Some(meganes[meganes.iter().position(|x| x.before == target)?].clone())
}

fn watch(mut expr: Vec<String>, meganes: &Vec<Megane>) -> Option<String> {
    while expr.len() > 1 {
        let mut index = 1;
        let mut is_solution = false;
        while index <= expr.len() {
            if let Some(matched) = search(meganes, expr.get(..index)?.to_vec()) {
                print!("Log: {:?}　-> ", &expr);
                expr = [vec![matched.after], expr[index..].to_vec()].concat();
                println!("{:?}", &expr);
                is_solution = true;
                break;
            } else {
                index += 1;
            }
        }
        if !is_solution {
            return None;
        }
    }
    Some(expr[0].clone())
}
