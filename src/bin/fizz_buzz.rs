fn main() {
    let mut n = 1;

    while n<100 {
        if n%3 == 0{
            print!("Fizz");
        }
        else if n%5 == 0 {
            print!("Buzz");
        }
        else {
            println!("\n{n}");
        }

        n += 1
        
    }
}
