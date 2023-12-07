fn main() {
    println!("Hello welcome world");
    fizz_buzz(301);
}

fn fizz_buzz(n:u32) -> (){
    for i in 1..=n{
        match i{
            i if &i % 15 == 0 => println!("fizz buzz"),
            i if &i % 3 == 0 => println!("fizz"),
            i if &i % 5 == 0 => println!("buzz"),
            _ => ()
        }
    }
}

