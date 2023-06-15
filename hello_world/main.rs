//here we define a function called main
//main function is special and it is always the first code that runs in every executable RUST program
//parameters always go throught the parenthesis
//Rust style is to indent using four spaces not a tab haha
//println! calls a macro like we are doing here , if we were calling a function instead we could have used println
// a semicolon indicates that this expression is over and the next one is ready to begin
//we must compile this program first in the cl and we do that by running rustc main.rs-> output is a binary executable ls to see it in our case called main
//Rust is an ahead of time compiled language meaning you can compile a program and give the executable to someone else and
//they can run it even without rust installed 
fn main(){
    println! ("Hello, WOrld!");
}