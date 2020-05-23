
const MAX_ENTITIES: u32 = 100_000;
const MAX_REQUESTS: u32 = 100000;

fn main() {
    /*
    ** This code can't work because x is immuable
    ** let x = 5;
    ** println!("The value of x is: {}", x);
    ** x = 6;
    ** println!("The value of x is: {}", x);
    */

    let mut mx = 5;
    println!("The value of mx is: {}", mx);
    mx = 6;
    println!("The value of mx is: {}", mx);

    let x = 5_;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    println!("The value of MAX_ENTITIES is: {}", MAX_ENTITIES);
    println!("The value of MAX_REQUESTS is: {}", MAX_REQUESTS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    /*
    ** Event your variable is mutable
    ** you can't change type, you could only change his value
    ** let mut spaces = "   ";
    ** spaces = spaces.len()
    */
}
