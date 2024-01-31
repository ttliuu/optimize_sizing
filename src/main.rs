use std::{collections::VecDeque, io};
fn main() {
    println!("Enter logic effort, separate by space");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    println!("Enter input cap");
    let mut cap_in = String::new();
    io::stdin().read_line(&mut cap_in).unwrap();

    println!("Enter output cap");
    let mut cap_out = String::new();
    io::stdin().read_line(&mut cap_out).unwrap();

    println!("Enter branch, separate by space");
    let mut branch = String::new();
    io::stdin().read_line(&mut branch).unwrap();

    
    let cap_in: f64 = cap_in.trim().parse().expect("NaN");
    let cap_out: f64 = cap_out.trim().parse().expect("NaN");

    let logic_efforts: Vec<f64> = line.trim().split(" ").map(|n|
        n.parse().expect("Not a number!")).collect();

    let branch: Vec<f64> = branch.trim().split(" ").map(|n|
        n.parse().expect("NaN")).collect();

    let total_g: f64 = logic_efforts.iter().product();
    let total_b: f64 = branch.iter().product();
    let total_h: f64 = cap_out / cap_in;
    let total_f = total_g * total_b * total_h;
    let optimal_f = total_f.powf(1.0 / (logic_efforts.len() as f64));
    println!("Optimal f is {}", optimal_f);

    let mut sizing: VecDeque<f64> = VecDeque::new();

    for g in logic_efforts.iter().rev().enumerate() {
        let cap = match sizing.len() {
            0 => cap_out,
            _ => { sizing.front().unwrap().clone() },
        };
        let cin = cap * g.1 * branch[branch.len() - 1 - g.0] / optimal_f;
        sizing.push_front(cin);
    }

    println!("Sizing: {:?}", sizing);

}
