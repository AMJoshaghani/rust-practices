use std::iter::zip;
use std::io;

fn main() {
    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    loop {
        let i = get_input("x: ");
        let j = get_input("y: ");
        if !i.is_empty() && !j.is_empty() {
            x.push(i.parse::<f32>().unwrap());
            y.push(j.parse::<f32>().unwrap());
        } else {
            println!("Exiting input mode.");
            break;
        }
    }
    println!("Data entered: x: {:?}, y: {:?}", x, y);
    let data: Data = Data { x_values: x, y_values: y };
    regression(&data);
}

fn regression(data: &Data) {
    let mut d = data.clone();
    let a1: f32 = d.sum(false, false) * d.sum(true, true) - d.sum(true, false) * d.sum_multiply();
    let a2: f32 = d.n() * d.sum(true, true) - d.sum(true, false).powf(2.0);

    let b1: f32 = d.n() * d.sum_multiply() - d.sum(true, false) * d.sum(false, false);
    let b2: f32 = d.n() * d.sum(true, true) - d.sum(true, false).powf(2.0);
    println!();
    println!("y = {:?} . x + {:?}", b1 / b2, a1 / a2);
}

fn get_input(prompt: &str) -> String {
    eprint!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

#[derive(Clone, Debug)]
struct Data {
    x_values: Vec<f32>,
    y_values: Vec<f32>,
}

impl Data {
    fn sum(&mut self, is_x: bool, to_2: bool) -> f32 {
        let dc = if is_x { &mut self.x_values } else { &mut self.y_values };
        let mut sum: f32 = 0.0;
        for i in dc.iter_mut() {
            sum += if to_2 { i.to_owned().powf(2.0) } else { i.to_owned() };
        }
        sum
    }
    fn sum_multiply(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for (i, j) in zip(&self.x_values, &self.y_values) {
            sum += i * j;
        }
        sum
    }
    fn n(&self) -> f32 {
        self.x_values.len() as f32
    }
}
