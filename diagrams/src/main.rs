use std::collections::HashMap;

const W: i32 = 100;
const H: i32 = 20;

fn main() {
    let mut x: HashMap<&str,i32> = HashMap::new();

    input_data(&mut x);
    create_diagram(x);
}

fn input_data(x: &mut HashMap<&str,i32>) {
    (*x).insert("зима",-23);
    (*x).insert("весна",10);
    (*x).insert("лето",27);
    (*x).insert("осень",17);
}

fn create_diagram(x: HashMap<&str,i32>) {
    let temp = x.clone();
    let max = temp.values().max();
    let temp = x.clone();
    let min = temp.values().min();
    
    if max!=None && min!=None {
        let matr: Vec<Vec<char>> = vec![vec!['-';W.try_into().unwrap()];H.try_into().unwrap()];
        let step: f64 = (max.unwrap()-min.unwrap()) as f64 / (H-1) as f64;

        print_legenda(x);
        print_diagram(matr, *max.unwrap() as f64,  *min.unwrap() as f64, step);
    }
    else {
        println!("Данные не найдены!");
    }

}

fn print_diagram(matr: Vec<Vec<char>>, mut max: f64, min: f64, step: f64) {
    let size: usize;

    if format!("{:.2}",max).chars().count()>format!("{:.2}",min).chars().count() {
        size = format!("{:.2}",max).chars().count();
    }
    else {
        size = format!("{:.2}",min).chars().count();
    }

    for row in &matr {
        let mut size_max = format!("{:.2}",max).chars().count();
        if size_max<size {
            while size_max<size {
                print!(" ");
                size_max+=1;
            }
        }
        print!("{:.2} |",max);
        for &value in row {
            print!("{}",value);
        }
        max-=step;
        println!("");
    }
}

fn print_legenda(x: HashMap<&str,i32>) {
    for _i in 0..W {
        print!("━");
    }
    println!("");
    print!("Данные: ");
    let mut count = 1;
    for (key,value) in &x {
        print!("    {}: ({} | {})",count,key,value);
        count+=1;
    }
    println!("");
    for _i in 0..W {
        print!("━");
    }
    println!("");
    println!("");
}