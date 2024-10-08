use std::collections::HashMap;

const MAX: i32 = 200;

fn main() {
    let mut y: i32 = 0;
    let mut step: i32 = 0;
    let mut x: HashMap<&str,i32> = HashMap::new();

    input_data(&mut y, &mut step, &mut x);
    create_diagram(y, step, x);
}

fn input_data(y: &mut i32, step: &mut i32, x: &mut HashMap<&str,i32>) {
    *y=-35;
    *step=5;
    (*x).insert("зима",-23);
    (*x).insert("весна",10);
    (*x).insert("лето",27);
    (*x).insert("осень",17);
}

fn create_diagram(y: i32, step: i32, x: HashMap<&str,i32>) {
    let max = x.values().max();
    
    if max!=None {
        let matr: Vec<Vec<char>> = vec![vec!['-';MAX.try_into().unwrap()];((max.unwrap()-y)/step).try_into().unwrap()];
        print_legenda(y, step, x);
        print_diagram(matr);
    }
    else {
        println!("Данные не найдены!");
    }

}

fn print_diagram(matr: Vec<Vec<char>>) {
    for row in &matr {
        for &value in row {
            print!("{}",value);
        }
        println!("");
    }
}

fn print_legenda(y: i32, step: i32, x: HashMap<&str,i32>) {
    for _i in 0..MAX {
        print!("━");
    }
    println!("");
    print!("y={}    step={}", y, step);
    let mut count = 1;
    for (key,value) in &x {
        print!("    {}: ({} | {})",count,key,value);
        count+=1;
    }
    println!("");
    for _i in 0..MAX {
        print!("━");
    }
    println!("");
    println!("");
}