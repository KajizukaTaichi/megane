use rustyline::DefaultEditor;

fn main() {
    println!("Megane: array-oriented computing model");
    let mut rl = DefaultEditor::new().unwrap();

    println!("Input data to be used");
    let mut data = vec![];
    loop {
        let inputed = rl.readline("> ").unwrap().trim().to_string();
        if inputed.is_empty() {
            break;
        }
        data.push(inputed);
    }
    dbg!(&data);

    println!("Input meganes to processing the data");
    let mut meganes = vec![];
    loop {
        let inputed = rl.readline("> ").unwrap().trim().to_string();
        if inputed.is_empty() {
            break;
        }
        let inputed: Vec<&str> = inputed.split(":").collect();
        meganes.push(Megane {
            before: inputed[0]
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
            after: inputed[1].trim().to_string(),
        });
    }
    dbg!(&meganes);

    let result = watch(data, &meganes);
    dbg!(result);
}

#[derive(Clone, Debug)]
struct Megane {
    before: Vec<String>,
    after: String,
}

fn search(meganes: &Vec<Megane>, target: Vec<String>) -> Option<Megane> {
    Some(meganes[meganes.iter().position(|x| x.before == target)?].clone())
}

fn watch(mut data: Vec<String>, meganes: &Vec<Megane>) -> Option<String> {
    while data.len() > 1 {
        dbg!(&data);
        let mut index = data.len();
        while index > 1 {
            if let Some(matched) = search(meganes, data.get(..index)?.to_vec()) {
                data = data
                    .join("\0")
                    .replace(&data[..index].join("\0"), &matched.after)
                    .split("\0")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                break;
            }
            index -= 1;
        }
    }
    Some(data[0].clone())
}
