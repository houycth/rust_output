mod add;

fn main() {
    let num1 = 5;
    let num2 = 3;
    let result = add::add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, result);
}