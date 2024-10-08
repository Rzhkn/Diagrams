use std::collections::HashMap;

const W: i32 = 100;
const H: i32 = 20;

fn main() {
    let mut x: HashMap<&str,f64> = HashMap::new();

    input_data(&mut x);
    create_diagram(&mut x);
}

fn input_data(x: &mut HashMap<&str,f64>) {
    (*x).insert("зима",-23.0);
    (*x).insert("весна",10.0);
    (*x).insert("лето",27.0);
    (*x).insert("осень",17.0);
}

fn create_diagram(x: &mut HashMap<&str,f64>) {
    let max = x.values().cloned().fold(f64::MIN, f64::max);
    let min = x.values().cloned().fold(f64::MAX, f64::min);
    
    let mut matr: Vec<Vec<char>> = vec![vec![' ';W.try_into().unwrap()];H.try_into().unwrap()];
    let step: f64 = (max-min) as f64 / (H-1) as f64;

    fill_diagram(&mut matr, x, min, step);
    print_diagram(&matr, x, max,  min, step);
}

fn fill_diagram(matr: &mut Vec<Vec<char>>, x: &mut HashMap<&str,f64>, min: f64, step: f64) {
    let count_st: i32 = (W-3*(x.len() as i32)-3) / (x.len() as i32);
    
    let mut ind_x = 0;
    for value in x.values() {
        let mut ind_y = min;
        let mut ind_matr = matr.len()-1;
        while ind_matr>0 && *value>=ind_y {
            for i in 0..count_st {
                matr[ind_matr][(3+3*ind_x+ind_x*count_st+i) as usize]='░';
            }
            ind_y+=step;
            ind_matr -= 1;
        }
        ind_x+=1;
    }
}

fn print_diagram(matr: &Vec<Vec<char>>, x: &mut HashMap<&str,f64>, mut max: f64, min: f64, step: f64) {
    let size: usize;

    if format!("{:.2}",max).chars().count()>format!("{:.2}",min).chars().count() {
        size = format!("{:.2}",max).chars().count();
    }
    else {
        size = format!("{:.2}",min).chars().count();
    }

    print_legenda(x, size);

    for row in matr {
        let mut size_max = format!("{:.2}",max).chars().count();
        if size_max<size {
            while size_max<size {
                print!(" ");
                size_max+=1;
            }
        }
        print!("{:.2} │",max);
        for &value in row {
            print!("{}",value);
        }
        max-=step;
        println!("");
    }
}

fn print_legenda(x: &HashMap<&str,f64>, size: usize) {
    let mut message = "Данные:".to_string();
    let mut count = 1;
    for (key,value) in x {
        message.push_str(&format!("    {}: ({} | {})",count,key,value).to_string());
        count+=1;
    }
    message.push_str("");

    for _i in 0..(W+size as i32) {
        print!("━");
    }
    println!("");

    for _i in 0..((W-message.chars().count() as i32)/2) {
        print!(" ");
    }
    print!("{}",message);
    for _i in 0..((W-message.chars().count() as i32)/2) {
        print!(" ");
    }
    println!("");

    for _i in 0..(W+size as i32) {
        print!("━");
    }
    println!("");
    println!("");
}