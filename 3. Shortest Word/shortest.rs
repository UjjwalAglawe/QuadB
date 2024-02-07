fn get_shortest<'a>(names: &Vec<&'a str>) -> Option<&'a str> {
    if names.len() == 0 {
        return None;
    }
    
    let mut n = names[0];
    for name in names {
        if name.len() < n.len() {
            n = name;
        }
    }
    return Some(n);
}

fn main() {
    let names:Vec<&str> = vec!["Dave", "Latoya", "Ben", "Jake"];
    
    println!("name: {:?}", &names);
    
    match get_shortest(&names) {
        Some(name) => {
            println!("Shortest: {}!", name);
        },
        None => {
            println!("No result!");
        }
    };        
}