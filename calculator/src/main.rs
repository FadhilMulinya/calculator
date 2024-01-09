use std::env::{args, Args};
fn main(){
    let mut args: Args = args();

    let first:String = args.nth(1).unwrap();
    let operator:String =args.nth(0).unwrap().chars().next().unwrap;
    let second:String = args.nth(0).unwrap();

    let first_number  = first.parse::<f32>().unwrap;
    let second_number = second.parse::<f32>().unwrap;

    println!("{} {} {}",first , operator, second );
}

fn operate(operator: char, first_number: f32, second_number: f32){
    match operator{
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ =>panic("invalid operator used"),
    }
}